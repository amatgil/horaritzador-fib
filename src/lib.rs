use std::{cmp::Ordering, fmt::Display, hash::Hash};

mod parsing;
pub use parsing::parse_raw_horari;

pub const RAW_HORARI: &str = include_str!("../input_data.txt");

#[derive(Debug, Clone, PartialEq)]

pub struct AssignaturaParse<'a> {
    nom: &'a str,
    grups: Vec<Grup>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sessio {
    dia: DiaSetmana,
    start: usize,
    finish: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Grup { // TODO: This gets cloned a lot, perhaps the Vec could be avoided?
    num: usize,
    llengua: Llengua,
    sessions: Vec<Sessio>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Llengua {
    #[default]
    Catala,
    Castella,
    Angles
}

#[derive(Debug, Clone)]
pub struct AssigDisplay<'a> {
    nom: &'a str,
    grup: usize,
    llengua: Llengua,
}


#[derive(Debug, Clone, Default)] pub struct ProtoHorari<'a>(Vec<(&'a str, Grup)>);
#[derive(Debug, Clone, Default)] pub struct Horari<'a>([Day<'a>; 5]);
#[derive(Debug, Clone, Default)] pub struct Day<'a>([Option<AssigDisplay<'a>>; 6]);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiaSetmana {
    Dilluns, Dimarts, Dimecres,
    Dijous, Divendres
}

impl<'a> Horari<'a> {
    fn comença_a_les_vuit(&self) -> bool {
        self.0.iter().any(|d| d.0[0].is_some())
    }
    fn quants_dies_comença_tard(&self) -> usize {
        self.0.iter().filter(|d| d.0[0].is_none()).count()
    }
    fn num_classes_angles(&self) -> usize {
        self.0.iter()
            .flat_map(|d| &d.0)                      
            .flatten()                               
            .filter(|h| h.llengua == Llengua::Angles)
            .count()                                 
    }

    fn te_dia_lliure(&self) -> bool {
        self.0.iter().any(|d| d.0.iter().all(|h| h.is_none()))
    }
}

// LA FUNCIÓ IMPORTANT
impl<'a> Ord for Horari<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.quants_dies_comença_tard().cmp(&other.quants_dies_comença_tard())
            .then(self.te_dia_lliure().cmp(&other.te_dia_lliure()))
            .then(self.num_classes_angles().cmp(&other.num_classes_angles()).reverse())
    }
}


impl Display for Llengua {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match &self {
            Llengua::Catala => 'ç',
            Llengua::Castella => 'ñ',
            Llengua::Angles => 'a',
        };
        write!(f, "{c}")
    }
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

impl PartialEq for Grup {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.llengua == other.llengua
    }
}

impl Eq for Grup {}

impl Hash for Grup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.num.hash(state);
        self.llengua.hash(state);
    }
}
    


impl<'a> Display for ProtoHorari<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for h in &self.0 {
            out.push_str(&format!("{}{} ", h.0, h.1.num));
        }
        write!(f, "{out}")
    }
}
impl<'a> Display for Horari<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = "|      |    Dilluns  |    Dimarts  |   Dimecres  |    Dijous   |  Divendres  |".to_string();
        out.push('\n');
        out.push_str(&"-".repeat(out.len()-1));
        out.push('\n');
        for h_i in 0..6 {
            out.push_str(&format!("|{: >6}|", h_i + 8));
            for d in &self.0 {
                match &d.0[h_i] {
                    Some(a) => out.push_str(&format!("{: >6}{: >4}({})|", a.nom, a.grup, a.llengua)),
                    None    => out.push_str(&format!("{: >13}|", "")),
                }
            }
            out.push('\n');
        }

        write!(f, "{}", out)
    }
}

impl<'a> PartialOrd for Horari<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<'a> Eq for Horari<'a>  {}
impl<'a> PartialEq for Horari<'a> {
    fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
}



impl<'a> TryFrom<ProtoHorari<'a>> for Horari<'a> {
    type Error = ();
    fn try_from(value: ProtoHorari<'a>) -> Result<Self, ()> {
        let mut s = Horari::default();

        for (nom, Grup { num, llengua, sessions }) in &value.0 {
            for Sessio { dia, start, finish } in sessions {
                let d = &mut s.0[*dia as usize];

                for hora in *start..*finish {
                    if d.0[hora - 8].is_some() { return Err(()) }

                    d.0[hora - 8] = Some(AssigDisplay {
                        nom,
                        grup: *num,
                        llengua: *llengua,
                    });
                }
            }
        }

        Ok(s)

    }

}

pub fn all_permutations<'a>(assigs: &[AssignaturaParse<'a>]) -> Vec<ProtoHorari<'a>> {
    let mut output = vec![];
    if assigs.len() <= 1 {
        for grup in &assigs[0].grups {
            output.push(ProtoHorari(vec![(assigs[0].nom, grup.clone())]));
        }
    } else {
        let rest = all_permutations(&assigs[1..]);
        for grup in &assigs[0].grups {
            for r in &rest {
                let mut new_line = r.clone();
                new_line.0.push((assigs[0].nom, grup.clone()));
                output.push(new_line);
            }

        }
    }

    output
}

