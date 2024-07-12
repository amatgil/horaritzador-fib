use horaritzador::*;
use nom::{error::convert_error, Err};

fn main() {
    let assignatures: Vec<AssignaturaParse> = match parse_raw_horari(RAW_HORARI) {
        Ok(parsed) => parsed.1,
        Err(Err::Error(e)) | Err(Err::Failure(e)) => {
            println!("{}", convert_error(RAW_HORARI, e));
            std::process::exit(1);
        },
        _ => unreachable!(),
    };

    println!("Getting permutations...");
    let perms = all_permutations(&assignatures);
    println!("Found {} permutations", perms.len());

    println!("Getting valid ones...");
    let mut hs: Vec<Horari> = perms.into_iter().filter_map(|ph| ph.try_into().ok()).collect();
    println!("There are {} valid horaris", hs.len());

    println!("Sorting the valid ones...");
    hs.sort_by(|a, b| b.cmp(a));

    let quants = 3;

    println!("Els millors, en teoria, son:");
    for i in 0..quants { println!("{}", hs[i]) }

    println!("I els pitjors, en teoria, son:");
    for i in 0..quants { println!("{}", hs[hs.len()-1-i]) }
    
}

