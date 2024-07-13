use crate::*;

use nom::{
    branch::alt, bytes::complete::{
        tag, take_till1
    }, character::complete::{
        self, multispace0,
    }, error::{context, VerboseError}, multi::{
        many1, separated_list1
    }, IResult

};

fn parse_sessio(input: &str) -> IResult<&str, Sessio, VerboseError<&str>> {
    let (input, _) = multispace0(input)?;

    let (input, dia) = context("getting dia de la setmana",
                               alt((
                                   tag("dilluns"), tag("dimarts"), tag("dimecres"),
                                   tag("dijous"), tag("divendres"),
                               )))(input)?;

    let (input, _) = multispace0(input)?;
    let (input, start) = context("getting start time of session", complete::u32)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, finish) = context("getting end time of session", complete::u32)(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, Sessio { dia: dia.try_into().unwrap(), start, finish }))}

fn parse_grup(input: &str) -> IResult<&str, Vec<Grup>, VerboseError<&str>> {
    // Numero
    let (input, _) = multispace0(input)?;
    let (input, nums) = context(
        "getting group numbers",
        separated_list1(tag(","), complete::u32))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, llengua) = context(
        "getting grup language ('catala', 'castella', 'angles')",
        alt((
            tag("catala"),
            tag("castella"),
            tag("angles"),
        )))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, sessions): (&str, Vec<Sessio>) =
        context("getting all sessions of group", many1(parse_sessio))(input)?;

    let grups: Vec<_> = nums.into_iter().map(|g| Grup {
        num: g as usize,
        llengua: llengua.try_into().unwrap(),
        sessions: sessions.clone(),
    }).collect();

    Ok((input, grups))
}

fn parse_assig(input: &str) -> IResult<&str, AssignaturaParse, VerboseError<&str>> {
    let (input, _) = multispace0(input)?;
    let (input, assig_name) = take_till1(|c| c == '\n')(input)?;

    let l = assig_name.len();
    let (assig_name, kind) = if assig_name.len() >= 3 {
        match &assig_name[l-2..] {
            "_L" => (&assig_name[..l-2], Some(AssigKind::Lab)),
            "_T" => (&assig_name[..l-2], Some(AssigKind::Teoria)),
            _    => (assig_name, None)
        }
    } else { (assig_name, None) };

    let (input, grups) = context(
        "parsing all of the groups",
        many1(context("parsing grup", parse_grup))
    )(input)?;

    let final_assig = AssignaturaParse {
        nom: assig_name,
        grups: grups.into_iter().flatten().collect(),
        kind
    };

    Ok((input, final_assig))
}

pub fn parse_raw_horari(input: &str) -> IResult<&str, Vec<AssignaturaParse>, VerboseError<&str>> {
    many1(parse_assig)(input)
}
