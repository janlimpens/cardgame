mod server;
use std::sync::Arc;

use server::Server;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

// use crate::{game::{Deck, Table}, card::{Facing, Stack}, client::Client};

// mod game;
// mod card;
// mod client;

// const values: [u8; 5] = [11, 12, 13, 10, 1];
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:6666").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);
    let server = Arc::new(Server::new());
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let server = Arc::clone(&server);
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                        server.handle(&msg);
                    }
                }
            }
        });
    }
}



// println!("let's play Schnapsen!");

// let client1 = Client::create("localhost:12345");
// client1.register_user("Hansi");
// client1.create_game("Schnapsen mit Hansi");
// client1.build_deck(vec!["Herz", "Schelle", "Laub", "Eichel"], vec!["Bube", "Dame", "König", "10", "Ass"], "Schwarz");
// let client2 = Client::create("localhost:12345");
