
use tokio::sync::mpsc;

async fn some_computation(input: u32) -> String {
    format!("the result of computation {}", input)
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
               
        for i in 0..10 {
            let res = some_computation(i).await;
            tx.send(res).await.unwrap();
        }
    });

    while let Some(res) = rx.recv().await {
        println!("got = {}", res);
    }
}

mod server;
// use std::sync::{Arc, Mutex};

// use server::Server;
// use tokio::{
//     io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
//     net::TcpListener,
//     sync::{broadcast},
// };

// // use crate::{game::{Deck, Table}, card::{Facing, Stack}, client::Client};

mod game;
mod card;
mod client;


// #[tokio::main]
// async fn main() {
//     let listener = TcpListener::bind("0.0.0.0:6666").await.unwrap();
    
//     let (tx, _rx) = broadcast::channel(10);
//     // let server = Arc::new(Mutex::new(Server::new()));
//     loop {
//         let (mut socket, addr) = listener.accept().await.unwrap();
//         let tx = tx.clone();
//         let mut rx = tx.subscribe();
//         // let server = Mutex::clone(&server);
//         tokio::spawn(async move {
//             let (reader, mut writer) = socket.split();
//             let mut reader = BufReader::new(reader);
//             let mut line = String::new();
//             loop {
//                 tokio::select! {
//                     result = reader.read_line(&mut line) => {
//                         if result.unwrap() == 0 {
//                             break;
//                         }
//                         tx.send((line.clone(), addr)).unwrap();
//                         line.clear();
//                     }
//                     result = rx.recv() => {
//                         let (msg, other_addr) = result.unwrap();
//                         if addr != other_addr {
//                             writer.write_all(msg.as_bytes()).await.unwrap();
//                         }
                        
//                         // server.get_mut().unwrap().handle(msg);
//                     }
//                 }
//             }
//         });
//     }

// }


// const values: [u8; 5] = [11, 12, 13, 10, 1];
// println!("let's play Schnapsen!");

// let client1 = Client::create("localhost:12345");
// client1.register_user("Hansi");
// client1.create_game("Schnapsen mit Hansi");
// client1.build_deck(vec!["Herz", "Schelle", "Laub", "Eichel"], vec!["Bube", "Dame", "König", "10", "Ass"], "Schwarz");
// let client2 = Client::create("localhost:12345");
