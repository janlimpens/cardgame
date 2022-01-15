#[macro_use] extern crate log;
extern crate env_logger;
mod server;
mod card;
mod game;

// use std::fmt::write;

use std::sync::Arc;

use server::Server;
use tokio::{
    io::{AsyncWriteExt},
    net::TcpListener,
    sync::{mpsc, watch, broadcast}
};
use crate::server::{Request, Response};
use std::env;
use env_logger::{Builder, Target};

fn init_logger() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
}

#[tokio::main]
async fn main() {
   
    init_logger();
    
    // let (tx, rx) = broadcast::channel(100);
    // let mut server = Server::new();
    // let req = serde_json::to_string(&Request::RegisterPlayer{player_name: "Captain Howdy".to_string()}).unwrap();
    // tx.send(req.).await.unwrap();
    // loop {
    //     tokio::spawn(async move {
    //         while let Some(request) = rx.recv().await {
    //             let request = serde_json::to_string(&request).unwrap();
    //             info!("Launching request {}", request);
    //             let response = server.handle(request);
    //             tx.send(response);
    //         }
    //     });
    // }   
    
    // let listener = TcpListener::bind("0.0.0.0:6666").await.expect("Failed to initialize listener");
    // let (server_tx, mut server_rx) = mpsc::channel::<Request>(100);
        
    // loop {
    //     let (mut socket, _addr) = listener.accept().await.unwrap();
    //     tokio::spawn(async move {
    //         let (_reader, mut writer) = socket.split();
    //         while let Some(request) = server_rx.recv().await {
    //             let request = serde_json::to_string(&request).unwrap();
    //             info!("Launching request {}", request);
    //             let response = server.handle(request);
    //             writer.write_all(response.as_bytes()).await.unwrap();
    //         }
    //     });
    // }

    
    
    let listener = TcpListener::bind("0.0.0.0:6666").await.expect("Failed to initialize listener");
    let (tx, _rx) = broadcast::channel::<String>(10);
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            loop {
                tokio::select! {
                    // result = reader.read_line(&mut line) => {
                    //     if result.unwrap() == 0 {
                    //         break;
                    //     }
                    //     tx.send((line.clone(), addr)).unwrap();
                    //     line.clear();
                    // }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
