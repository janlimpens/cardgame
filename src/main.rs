
mod server;
mod card;
mod game;

// use std::fmt::write;

use server::Server;
use tokio::{
    io::{AsyncWriteExt},
    net::TcpListener,
    sync::{mpsc}
};
use crate::server::Request;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:6666").await.unwrap();
    let (tx, mut rx) = mpsc::channel::<Request>(100);
    tx.send(Request::RegisterPlayer{player_name: "Captain Howdy".to_string()}).await.unwrap();
    let mut server = Server::new();
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        //let rx = rx. ??
        tokio::spawn(async move {
            let (_reader, mut writer) = socket.split();
            while let Some(request) = rx.recv().await {
                let request = serde_json::to_string(&request).unwrap();
                let response = server.handle(request);
                writer.write_all(response.as_bytes()).await.unwrap();
            }
        });
    }

    
    // let (tx, _rx) = broadcast::channel(10);
    // loop {
    //     let (mut socket, addr) = listener.accept().await.unwrap();
    //     let tx = tx.clone();
    //     let mut rx = tx.subscribe();
    //     tokio::spawn(async move {
    //         let (reader, mut writer) = socket.split();
    //         let mut reader = BufReader::new(reader);
    //         let mut line = String::new();
    //         loop {
    //             tokio::select! {
    //                 result = reader.read_line(&mut line) => {
    //                     if result.unwrap() == 0 {
    //                         break;
    //                     }
    //                     tx.send((line.clone(), addr)).unwrap();
    //                     line.clear();
    //                 }
    //                 result = rx.recv() => {
    //                     let (msg, other_addr) = result.unwrap();
    //                     if addr != other_addr {
    //                         writer.write_all(msg.as_bytes()).await.unwrap();
    //                     }
    //                 }
    //             }
    //         }
    //     });
    // }
}
