use std::collections::HashMap;


const RAW_HORARI: &str = include_str!("../input_data.txt");

#[derive(Debug, Clone, Copy)]
enum Llengua {
    Catala,
    Castella,
    Angles
}
impl TryFrom<&str> for Llengua {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "catala" => Ok(Llengua::Catala),
            "castella" => Ok(Llengua::Castella),
            "angles" => Ok(Llengua::Angles),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum DiaSetmana {
    Dilluns, Dimarts, Dimecres,
    Dijous, Divendres
}

impl TryFrom<&str> for DiaSetmana {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "dilluns"   => Ok(Self::Dilluns),
            "dimarts"   => Ok(Self::Dimarts),
            "dimecres"  => Ok(Self::Dimecres),
            "dijous"    => Ok(Self::Dijous),
            "divendres" => Ok(Self::Divendres),
            _           => Err(())
        }
    }
}

#[derive(Debug, Clone)]
struct Assignatura {
    nom: String,
    grups: Vec<Grup>,
}

#[derive(Debug, Clone, Copy)]
struct Sessio {
    dia: DiaSetmana,
    start: usize,
    finish: usize,
}

#[derive(Debug, Clone)]
struct Grup {
    num: usize,
    llengua: Llengua,
    sessions: Vec<Sessio>,
}

struct Day([Option<Assignatura>; 6]);

struct Horari([Day; 5]);

fn main() {
    let mut assignatures: Vec<Assignatura> = parse_raw_horari(RAW_HORARI).expect("Could not parse horari").1;

    dbg!(assignatures);
}

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
        tag("dilluns"),
        tag("dimarts"),
        tag("dimecres"),
        tag("dijous"),
        tag("divendres"),
    ))(input)?;

    dbg!(input);
    let (input, _) = space0(input)?;
    let (input, start) = complete::u32(input)?;
    let (input, _) = space0(input)?;
    let (input, end) = complete::u32(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((input, Sessio { dia: dia.try_into().unwrap(),
                        start: start as usize,
                        finish: end as usize }))
}
fn parse_grup(input: &str) -> IResult<&str, Grup> {
    let newl_tab = tag("\n\t");
    let (input, _) = newl_tab(input)?;

    // Numero
    let (input, num) = complete::u32(input)?;
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
    dbg!(input);
    let mut input = input;
    for _ in 0..n_sessions {
        let (newinput, sessio) = parse_sessio(input)?;
        sessions.push(sessio);
        input = newinput;
    }
    dbg!(n_sessions, &sessions);


    Ok((input, Grup {
        num: num as usize,
        llengua: llengua.try_into().unwrap(),
        sessions
    }))
}

fn parse_assig(input: &str) -> IResult<&str, Assignatura> {
    let (input, assig_name) = take_till1(|c| c == '\n')(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _num_grups) = complete::u32(input)?;

    let (input, grups) = many1(parse_grup)(input)?;

    let final_assig = Assignatura {
        nom: assig_name.into(),
        grups,
    };

    Ok((dbg!(input), final_assig))
}

fn parse_raw_horari(input: &str) -> IResult<&str, Vec<Assignatura>> {
    let (input, output) = separated_list0(tag("\n"), parse_assig)(input)?;

    Ok((input, output))
}
