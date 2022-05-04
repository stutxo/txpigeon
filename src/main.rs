use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, process};
use std::io::{Write, BufReader};
use bitcoin::network::message_blockdata::Inventory;
use rand::Rng;
use bitcoin::{Transaction};
use bitcoin::consensus::{encode, Decodable, deserialize};
use bitcoin::hashes::hex::FromHex;
use bitcoin::network::{address, constants, message, message_network};


fn main() -> std::io::Result<()>{
    // This example establishes a connection to a Bitcoin node, sends the intial
    // "version" message, waits for the reply, and finally closes the connection.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("not enough arguments");
        process::exit(1);
    }

    let str_address = &args[1];

    let address: SocketAddr = str_address.parse().unwrap_or_else(|error| {
        eprintln!("Error parsing address: {:?}", error);
        process::exit(1);
    });

    let version_message = build_version_message(address);

    let first_message = message::RawNetworkMessage {
        magic: constants::Network::Testnet.magic(),
        payload: version_message,
    };    

    //create tx inv message
    let txhex: Transaction = deserialize(&Vec::from_hex("020000000001012125bdeec17f5c6446a334df9fe2e5aeec13c496d3979a50620d3c07054a94510000000000feffffff02905f0100000000001976a914344a0f48ca150ec2b903817660b9b68b13a6702688ac6526000000000000160014acaecb6062c078f9f4c272df0349330a9398787102473044022042b1e9acb3f07e514625cb50033b3075bbe82ef99d4a3590bc25dc65541dd1820220364c3cc9c0dbd50db7b540e2f21e6267ebbdb03bc3f7c6708e35989e08c09e1b012102469e82060d46d012506786a8069ac461710cc97ff9f5bc6f7472bb252a1c685036842100").unwrap()).unwrap();
    let txid = Inventory::Transaction(txhex.txid());
    let mut txvec = Vec::new();
    txvec.push(txid);

    let inv_message = message::RawNetworkMessage {
        magic: constants::Network::Testnet.magic(),
        payload: message::NetworkMessage::Inv(txvec),
    };

    
    
    //create tx send message
    let tx_message = message::RawNetworkMessage {
        magic: constants::Network::Testnet.magic(),
        payload: message::NetworkMessage::Tx(txhex),
    };

    
    
    if let Ok(mut stream) = TcpStream::connect(address) {
        // Send the message
        let _first = stream.write_all(encode::serialize(&first_message).as_slice());
        println!("Sent version message");
                
        // Setup StreamReader
        let read_stream = stream.try_clone().unwrap();
        let mut stream_reader = BufReader::new(read_stream);
        loop {
            // Loop and retrieve new messages
            let reply = message::RawNetworkMessage::consensus_decode(&mut stream_reader).unwrap();
            match reply.payload {
                message::NetworkMessage::Version(_) => {
                    println!("Received version message: {:?}", reply.payload);

                    let second_message = message::RawNetworkMessage {
                        magic: constants::Network::Testnet.magic(),
                        payload: message::NetworkMessage::Verack,
                    };

                    let _ = stream.write_all(encode::serialize(&second_message).as_slice());
                    println!("Sent verack message");
                }
                message::NetworkMessage::Verack => {
                    println!("Received verack message: {:?}", reply.payload);
                    //send inv message
                    let _ = stream.write_all(encode::serialize(&inv_message).as_slice());
                    println!("Sent inv message {:?}", &inv_message);
                    
                }
                message::NetworkMessage::GetData(_) => {
                    println!("Received GetData message: {:?}", reply.payload);            
                   
                    let txidreply = txid;
                    let mut txvecreply = Vec::new();
                    txvecreply.push(txidreply);

                    if reply.payload == message::NetworkMessage::GetData(txvecreply) {
                        stream.write_all(encode::serialize(&tx_message).as_slice())?;
                        println!("broadcast TX {:?}", &tx_message);
                    }
                } 
                message::NetworkMessage::Ping(nonce) => {
                    println!("Received ping message: {:?}", reply.payload);

                    let pong_message = message::RawNetworkMessage {
                        magic: constants::Network::Testnet.magic(),
                        payload: message::NetworkMessage::Pong(nonce),
                    };

                    let _ = stream.write_all(encode::serialize(&pong_message).as_slice());
                    println!("Sent pong message {:?}", &pong_message);

                } 
                _ => {
                    println!("Received unknown message: {:?}", reply.payload);
                    
                }
            }
        }
        
    } else {
        eprintln!("Failed to open connection");
        process::exit(1);
    }
}



fn build_version_message(address: SocketAddr) -> message::NetworkMessage {
    // Building version message, see https://en.bitcoin.it/wiki/Protocol_documentation#version
    let my_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

    // "bitfield of features to be enabled for this connection"
    let services = constants::ServiceFlags::NONE;

    // "standard UNIX timestamp in seconds"
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_secs();

    // "The network address of the node receiving this message"
    let addr_recv = address::Address::new(&address, constants::ServiceFlags::NONE);

    // "The network address of the node emitting this message"
    let addr_from = address::Address::new(&my_address, constants::ServiceFlags::NONE);

    // "Node random nonce, randomly generated every time a version packet is sent. This nonce is used to detect connections to self."
    let nonce: u64 = rand::thread_rng().gen();

    // "User Agent (0x00 if string is 0 bytes long)"
    let user_agent = String::from("testing");

    // "The last block received by the emitting node"
    let start_height: i32 = 0;

    // Construct the message
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
