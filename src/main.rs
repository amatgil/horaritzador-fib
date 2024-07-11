use horaritzador::*;

fn main() {
    let assignatures: Vec<AssignaturaParse> = parse_raw_horari(RAW_HORARI).expect("Could not parse horari").1;

    println!("Getting permutations...");
    let perms = all_permutations(&assignatures);
    println!("Found {} permutations", perms.len());

    println!("Getting valid ones...");
    let mut hs: Vec<Horari> = perms.into_iter().filter_map(|ph| ph.try_into().ok()).collect();
    println!("There are {} valid horaris", hs.len());

    println!("Sorting the valid ones...");
    hs.sort_by(|a, b| b.cmp(a));
    println!("Els millors, en teoria, son:");

    let quants = 3;
    for i in 0..quants {
        println!("{}", hs[i])
    }

    println!("I els pitjors, en teoria, son:");
    for i in 0..quants {
        println!("{}", hs[hs.len() - 1 - i]);
    }
    
}

