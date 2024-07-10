use std::{cmp::Ordering, collections::{HashMap, HashSet}, default, fmt::Display, hash::Hash};

mod parsing;
use parsing::parse_raw_horari;

const RAW_HORARI: &str = include_str!("../input_data.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Llengua {
    #[default]
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

#[derive(Debug, Clone, Default)]
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
impl Eq for GrupParse {}

impl Hash for GrupParse {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.num.hash(state);
        self.llengua.hash(state);
    }
}
    

#[derive(Debug, Clone)]
struct AssigDisplay {
    nom: String,
    grup: usize,
    llengua: Llengua,
}

#[derive(Debug, Clone, Default)]
struct ProtoHorari(Vec<(String, GrupParse)>);
#[derive(Debug, Clone, Default)]
struct Horari([Day; 5]);
#[derive(Debug, Clone, Default)]
struct Day([Option<AssigDisplay>; 6]);

impl Display for ProtoHorari {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for h in &self.0 {
            out.push_str(&format!("{}{} ", h.0, h.1.num));
        }
        write!(f, "{out}")
    }
}
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
    fn add_assig(&mut self, sess: &Sessio, assig: &AssigDisplay) -> Result<(), ()> {
        let i = sess.dia as usize;
        for h in sess.start..sess.finish {
            if self.0[i].0[h - 8].is_some() { return Err(()); }
            self.0[i].0[h - 8] = Some(assig.clone());
        }
        Ok(())
    }
    fn comença_a_les_vuit(&self) -> bool {
        self.0.iter().any(|d| d.0[0].is_some())
    }
    fn te_dia_lliure(&self) -> bool {
        self.0.iter().any(|d| d.0.iter().all(|h| h.is_none()))
    }
    fn generate_from_groups(map: HashMap<GrupParse, (String, Vec<Sessio>)>) -> Option<Self> {
        let mut h = Horari::default();
        for (grup, (nom, sessions)) in map.into_iter() {
            let display = AssigDisplay {
                nom,
                grup: grup.num,
                llengua: grup.llengua,
            };
            for sessio in sessions {
                h.add_assig(&sessio, &display).ok()?;
            }
        }
        Some(h)
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
        self.comença_a_les_vuit().cmp(&other.comença_a_les_vuit())
            .then(self.te_dia_lliure().cmp(&other.te_dia_lliure()).reverse())
    }
}

fn all_permutations(assigs: &[AssignaturaParse]) -> Vec<ProtoHorari> {
    let mut output = vec![];
    if assigs.len() <= 1 {
        for grup in &assigs[0].grups {
            output.push(ProtoHorari(vec![(assigs[0].nom.clone(), grup.clone())]));
        }
    } else {
        let rest = all_permutations(&assigs[1..]);
        for grup in &assigs[0].grups {
            for r in &rest {
                let mut new_line = r.clone();
                new_line.0.push(dbg!(assigs[0].nom.clone(), grup.clone()));
                output.push(new_line);
            }

        }
    }

    output
}

fn main() {
    use itertools::Itertools;

    let mut assignatures: Vec<AssignaturaParse> = parse_raw_horari(RAW_HORARI).expect("Could not parse horari").1;

    //dbg!(&assignatures[0]);
    //let perms = all_permutations(&assignatures[0..1]);
    let input = vec![
        AssignaturaParse { nom: "A".into(), grups: vec![
            GrupParse { num: 1, ..Default::default() },
            GrupParse { num: 2, ..Default::default() },
            GrupParse { num: 3, ..Default::default() },
        ]},
        AssignaturaParse { nom: "B".into(), grups: vec![
            GrupParse { num: 2, ..Default::default() },
            GrupParse { num: 3, ..Default::default() },
        ]},
        AssignaturaParse { nom: "C".into(), grups: vec![
            GrupParse { num: 1, ..Default::default() },
        ]},
        AssignaturaParse { nom: "D".into(), grups: vec![
            GrupParse { num: 2, ..Default::default() },
            GrupParse { num: 3, ..Default::default() },
            GrupParse { num: 4, ..Default::default() },
        ]},
    ];
    let perms = all_permutations(&input);
    for p in &perms {
        println!("{p}");
    }
    dbg!(perms.len());



    
}

