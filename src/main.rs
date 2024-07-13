use horaritzador::*;
use nom::{error::convert_error, Err};
use std::time::Instant; // Seeing how long things took, unnecessary but neat

fn main() {
    println!("Parsing input data..");
    let parsing_start = Instant::now();
    let assignatures: Vec<AssignaturaParse> = match parse_raw_horari(RAW_HORARI) {
        Ok(parsed) => parsed.1,
        Err(Err::Error(e)) | Err(Err::Failure(e)) => {
            println!("{}", convert_error(RAW_HORARI, e));
            std::process::exit(1);
        },
        _ => unreachable!(),
    };

    for AssignaturaParse { nom, grups, .. } in assignatures.iter().filter(|a| a.kind == Some(AssigKind::Teoria)) {
        if let Some(i) = grups.iter().position(|g| g.num % 10 != 0) {
            println!("[ERROR]: els grups de les classes de teoria han de ser 0 mod 10 (l'assignatura '{}', grup {}, no ho compleix això)",
                     nom,
                     grups[i].num
            );
            std::process::exit(2);
        }
    }

    let parsing_time = parsing_start.elapsed();
    println!("Parsed input file in {}s", parsing_time.as_secs_f32());

    println!("Getting permutations...");
    let perms_start = Instant::now();
    let perms = all_permutations(&assignatures);
    let perms_time = perms_start.elapsed();
    println!("Found {} permutations in {}s", perms.len(), perms_time.as_secs_f32());



    println!("Getting valid ones...");
    let filter_start = Instant::now();
    let mut hs: Vec<Horari> = perms.into_iter().filter_map(|ph| ph.try_into().ok()).collect();
    let filter_time = filter_start.elapsed();
    println!("There are {} valid horaris in {}s", hs.len(), filter_time.as_secs_f32());



    println!("Sorting the valid ones...");
    let sort_start = Instant::now();
    hs.sort_by(|a, b| b.cmp(a)); // Backwards
    let sort_time = sort_start.elapsed();
    println!("Sorting done in {}s", sort_time.as_secs_f32());



    let quants = 3;

    println!("Els millors, en teoria, son:");
    for h in hs.iter().take(quants) { println!("{h}") }

    println!("I els pitjors, en teoria, son:");
    for h in hs.iter().rev().take(quants) { println!("{h}") }


    println!("Times taken:");
    println!("\t{}s: getting all permutations", perms_time.as_secs_f32());
    println!("\t{}s: filtering out invalid ones", filter_time.as_secs_f32());
    println!("\t{}s: sorting from best to worse", sort_time.as_secs_f32());
    println!("\nTotal time is: {}", (perms_time + filter_time + sort_time).as_secs_f32());
    
}

