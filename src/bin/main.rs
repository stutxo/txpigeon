extern crate broadcast;
extern crate rand;


use broadcast::{dns::dns_seed, run::egg};
use std::{env, process};
use rand::prelude::SliceRandom;

fn main() -> std::io::Result<()>{

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("not enough arguments");
        process::exit(1);
    }

    let tx = &args[1];


    let dns = dns_seed();
    let seed: Vec<_> = dns
    .choose_multiple(&mut rand::thread_rng(), 1)
    .collect();
    
    let rngseed = *seed[0];
    // /println!("{:?}", rngseed);

    let address = rngseed;

    egg(address, tx)?;
        
    Ok(())
}


