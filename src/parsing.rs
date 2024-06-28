use crate::*;

use nom::{
    IResult,
    bytes::complete::{
        take_till1,
        tag,
    },
    character::complete::{
        self, digit0, space0
    },
    branch::alt,
    multi::{
        separated_list0,
        many1,
    },

};

fn parse_sessio(input: &str) -> IResult<&str, Sessio> {
    let (input, _) = tag("\t")(input)?;

    let (input, dia) = alt((
        tag("dilluns"), tag("dimarts"), tag("dimecres"),
        tag("dijous"), tag("divendres"),
    ))(input)?;

    let (input, _) = space0(input)?;
    let (input, start) = complete::u32(input)?;
    let (input, _) = space0(input)?;
    let (input, end) = complete::u32(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((input, Sessio { dia: dia.try_into().unwrap(),
                        start: start as usize,
                        finish: end as usize }))
}
fn parse_grup(input: &str) -> IResult<&str, Vec<GrupParse>> {
    let newl_tab = tag("\n\t");
    let (input, _) = newl_tab(input)?;

    // Numero
    let (input, nums) = separated_list0(tag(","), complete::u32)(input)?;
    let (input, _) = newl_tab(input)?;

    // Llengua

    let (input, llengua) = alt((
        tag("catala"),
        tag("castella"),
        tag("angles"),
    ))(input)?;
    let (input, _) = newl_tab(input)?;

    // Dies
    let (input, n_sessions) = complete::u32(input)?;
    let (input, _) = tag("\n")(input)?;

    let mut sessions = Vec::new();
    let mut input = input;
    for _ in 0..n_sessions {
        let (newinput, sessio) = parse_sessio(input)?;
        sessions.push(sessio);
        input = newinput;
    }


    let grups: Vec<_> = nums.into_iter().map(|g| GrupParse {
        num: g as usize,
        llengua: llengua.try_into().unwrap(),
        sessions: sessions.clone(),
    }).collect();

    Ok((input, grups))
}

fn parse_assig(input: &str) -> IResult<&str, AssignaturaParse> {
    let (input, assig_name) = take_till1(|c| c == '\n')(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _num_grups) = complete::u32(input)?;

    let (input, grups) = many1(parse_grup)(input)?;

    let final_assig = AssignaturaParse {
        nom: assig_name.into(),
        grups: grups.into_iter().flatten().collect(),
    };

    Ok((input, final_assig))
}

pub fn parse_raw_horari(input: &str) -> IResult<&str, Vec<AssignaturaParse>> {
    let (input, output) = separated_list0(tag("\n"), parse_assig)(input)?;

    Ok((input, output))
}
