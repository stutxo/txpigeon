use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process;
use std::io::{Write, BufReader};
use bitcoin::network::message_blockdata::Inventory;
use rand::Rng;
use bitcoin::{Transaction};
use bitcoin::consensus::{encode, Decodable, deserialize};
use bitcoin::hashes::hex::FromHex;
use bitcoin::network::{address, message, constants, message_network};
use rand::prelude::SliceRandom;
use crate::dns::dns_seed;


pub fn sendtx(address:SocketAddr, tx: &String) -> std::io::Result<()> {
        
    if let Ok (mut stream) = TcpStream::connect(address) {

    let version_message = build_version_message(address);

    let first_message = message::RawNetworkMessage {
    magic: constants::Network::Testnet.magic(),
    payload: version_message,
        };    

    let txhex: Transaction = deserialize(&Vec::from_hex(&tx).unwrap()).unwrap();
    let txid = Inventory::Transaction(txhex.txid());
    let mut txvec = Vec::new();
    txvec.push(txid);

    let inv_message = message::RawNetworkMessage {
    magic: constants::Network::Testnet.magic(),
    payload: message::NetworkMessage::Inv(txvec),
        };

let tx_message = message::RawNetworkMessage {
    magic: constants::Network::Testnet.magic(),
    payload: message::NetworkMessage::Tx(txhex),
};
    let _first  = stream.write_all(encode::serialize(&first_message).as_slice());
    println!("Sent version message to {:?}", address);
            
    let read_stream = stream.try_clone().unwrap();
    let mut stream_reader = BufReader::new(read_stream);

    loop {
        let reply = message::RawNetworkMessage::consensus_decode(&mut stream_reader).unwrap();
        match reply.payload {
            message::NetworkMessage::Version(_) => {
                println!("Received version message from: {:?}", address);

                let second_message = message::RawNetworkMessage {
                    magic: constants::Network::Testnet.magic(),
                    payload: message::NetworkMessage::Verack,
                };

                let _ = stream.write_all(encode::serialize(&second_message).as_slice());
                //println!("Sent verack message");
            }
            message::NetworkMessage::Verack => {                
                let _ = stream.write_all(encode::serialize(&inv_message).as_slice());
                println!("Sent inv message to {:?}", address);                
            }
            message::NetworkMessage::GetData(_) => {
                let txidreply = txid;
                let mut txvecreply = Vec::new();
                txvecreply.push(txidreply);

                if reply.payload == message::NetworkMessage::GetData(txvecreply) {
                    println!("Received GetData message: {:?}", reply.payload); 
                    stream.write_all(encode::serialize(&tx_message).as_slice())?;
                    println!("TX broadcast to {:?}", address);
                    process::exit(1);
                }
                
            } 
            message::NetworkMessage::Ping(nonce) => {
                //println!("Received ping message: {:?}", reply.payload);

                let pong_message = message::RawNetworkMessage {
                    magic: constants::Network::Testnet.magic(),
                    payload: message::NetworkMessage::Pong(nonce),
                };

                let _ = stream.write_all(encode::serialize(&pong_message).as_slice());
                //println!("Sent pong message {:?}", &pong_message);

            } 
            _ => {
                println!("Received unknown message: {:?}", reply.payload);
                
            }
            
        }
    }
    
    } else {
    eprintln!("Failed to open connection, trying again...");

    let dns = dns_seed();
    let seed: Vec<_> = dns
    .choose_multiple(&mut rand::thread_rng(), 1)
    .collect();
    let rngseed = *seed[0];
    let address = rngseed;

    sendtx(address, tx)?;
    Ok(())
    }
}

fn build_version_message(address: SocketAddr) -> message::NetworkMessage {
   
    let my_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

    let services = constants::ServiceFlags::NONE;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_secs();

    let addr_recv = address::Address::new(&address, constants::ServiceFlags::NONE);

    let addr_from = address::Address::new(&my_address, constants::ServiceFlags::NONE);

    let nonce: u64 = rand::thread_rng().gen();

    let user_agent = String::from("testing");

    let start_height: i32 = 0;

    message::NetworkMessage::Version(message_network::VersionMessage::new(
        services,
        timestamp as i64,
        addr_recv,
        addr_from,
        nonce,
        user_agent,
        start_height,
    ))
}