use log::{info, trace};
use std::net::{SocketAddr, ToSocketAddrs};


const TEST_SEEDER: [&str; 4] = [
    "testnet-seed.bitcoin.jonasschnelli.ch",
    "seed.tbtc.petertodd.org",
    "seed.testnet.bitcoin.sprovoost.nl",
    "testnet-seed.bluematt.me",
];

pub fn dns_seed() -> Vec<SocketAddr> {
    let mut seeds = Vec::new();
        for seedhost in TEST_SEEDER.iter() {
            if let Ok(lookup) = (*seedhost, 18333).to_socket_addrs() {
                for host in lookup {
                    seeds.push(host);
                }
            } else {
                trace!("{} did not answer", seedhost);
            }
        }
        info!("received {} DNS seeds", seeds.len());
    seeds
}

/*const HARDCODE: [&str; 5] = [
"13.112.33.52:18333",
"18.207.102.214:18333",
"40.118.228.187:18333",
"23.239.26.175:18333",
"23.234.205.19:18333",
];

pub fn hardseeds() -> Vec<SocketAddr> {
    let mut seeds = Vec::new();
    for seedhost in HARDCODE.iter() {
        if let Ok(lookup) = (*seedhost).to_socket_addrs() {
            for host in lookup {
                seeds.push(host);
            }
        } else {
            trace!("{} did not answer", seedhost);
        }
    }
    info!("received {} DNS seeds", seeds.len());
seeds
}*/