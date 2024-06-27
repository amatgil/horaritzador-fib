use std::{cmp::Ordering, collections::HashMap};

mod parsing;
use parsing::parse_raw_horari;

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

#[derive(Debug, Clone)]
struct AssigDisplay {
    nom: String,
    grup: usize,
    llengua: Llengua,
}

#[derive(Debug, Clone, Default)]
struct Horari([Day; 5]);
#[derive(Debug, Clone, Default)]
struct Day([Option<AssigDisplay>; 6]);

impl Horari {
    fn add_assig(&mut self, sess: &Sessio, assig: &AssigDisplay) {
        let i = sess.dia as usize;
        for h in sess.start..sess.finish {
            self.0[i].0[h - 8] = Some(assig.clone());
        }
    }
    fn comença_a_les_vuit(&self) -> bool {
        self.0.iter().any(|d| d.0[0].is_some())
    }
}

impl PartialOrd for Horari {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Eq for Horari  {}
impl PartialEq for Horari {
    fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
}


impl Ord for Horari {
    fn cmp(&self, other: &Self) -> Ordering {
        if       self.comença_a_les_vuit() && !other.comença_a_les_vuit() { Ordering::Less }
        else if !self.comença_a_les_vuit() &&  other.comença_a_les_vuit() { Ordering::Greater }
        else { // Cap dels dos comença a les 8
            todo!()
        }
    }
}


fn main() {
    let mut assignatures: Vec<Assignatura> = parse_raw_horari(RAW_HORARI).expect("Could not parse horari").1;

    let mut h1 = Horari::default();
    let mut h2 = Horari::default();
    
}

