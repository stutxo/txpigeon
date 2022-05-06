use broadcast::{dns::dns_seed, run::sendtx};
use rand::prelude::SliceRandom;
use std::{env, process};

fn main()  -> std::io::Result<()> {

    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("not enough arguments");
        process::exit(1);
    }
    let tx = args.remove(1);


    let dns = dns_seed();
    let seed: Vec<_> = dns
    .choose_multiple(&mut rand::thread_rng(), 1)
    .collect();
    let rngseed = *seed[0];
    let address = rngseed;

    loop {
        sendtx(address, &tx)?;
    }
}


