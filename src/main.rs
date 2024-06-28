use std::{cmp::Ordering, collections::{HashMap, HashSet}, fmt::Display};

mod parsing;
use parsing::parse_raw_horari;

const RAW_HORARI: &str = include_str!("../input_data.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
struct AssignaturaParse {
    nom: String,
    grups: Vec<GrupParse>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Sessio {
    dia: DiaSetmana,
    start: usize,
    finish: usize,
}

#[derive(Debug, Clone)]
struct GrupParse {
    num: usize,
    llengua: Llengua,
    sessions: Vec<Sessio>,
}

impl PartialEq for GrupParse {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num &&
            self.llengua == other.llengua
    }
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

impl Display for Horari {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = "|      | Dilluns | Dimarts | Dimecres | Dijous | Divendres |".to_string();
        out.push('\n');
        out.push_str("----------------------------------------------------------------");
        out.push('\n');
        for h_i in 0..6 {
            out.push_str(&format!("|{: >6}|", h_i + 8));
            for d in &self.0 {
                match &d.0[h_i] {
                    Some(a) => out.push_str(&format!(" {: >4} {: >2} |", a.nom, a.grup)),
                    None    => out.push_str(&format!("         |", )),
                }
            }
            out.push('\n');
        }

        write!(f, "{}", out)
    }
}

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
    fn generate_from_groups(gs: &[(String, GrupParse)]) -> Option<Self> {
        let mut h = Self::default();
        for (s, g) in gs {
            
        }
        todo!()
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
            Ordering::Equal // TODO: Add more ordering criteria
        }
    }
}

fn main() {
    use itertools::Itertools;

    let mut assignatures: Vec<AssignaturaParse> = parse_raw_horari(RAW_HORARI).expect("Could not parse horari").1;

    let noms: Vec<_> = assignatures.iter().map(|a| a.nom.clone()).collect();
    let grups: Vec<Vec<_>> = assignatures.iter().map(|a| a.grups.clone()).collect();

    //let mut x = 0;
    //for g in grups {
    //    //dbg!(g);
    //    println!("{}", g.len());
    //    x += 1;
    //}
    //dbg!(x);

    
    let mut h = Horari::default();

    h.add_assig(
        &Sessio {
            dia: DiaSetmana::Dilluns,
            start: 8,
            finish: 10,
        },
        &AssigDisplay {
            nom: "EC".into(),
            grup: 11,
            llengua: Llengua::Catala,
        }
    );
    h.add_assig(
        &Sessio {
            dia: DiaSetmana::Dimarts,
            start: 9,
            finish: 11,
        },
        &AssigDisplay {
            nom: "IC_T".into(),
            grup: 18,
            llengua: Llengua::Castella,
        }
    );
    println!("{h}");


    
}

