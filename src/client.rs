#![allow(dead_code)]
#![allow(unused_variables)]

// use dialectic::{self, Session};
// use dialectic_tokio_mpsc as mpsc;
// use tokio;
// use tokio::task;
// use std::io;

// type RegisterPlayer = Session! {
//     send String;
// };

pub struct Client {
    player: String,
    server: String,
    token: String,
}

// impl Client {
//     pub fn create(address:&str) -> Client {
//         Client {
//             player: "".to_string(),
//             server: address.to_string(),
//             token: "".to_string()
//         }
//     }

//     pub async fn register_user(&self, name: &str) -> Result<&str, &str> {
//         let (c1, c2) = RegisterPlayer::channel(|| mpsc::channel(1));
//         let t = tokio::spawn(async move {
//             c1.send(name.to_string()).await?;
//             let (response, c2) = c2.recv().await?;
//             self.player = response;
//             Ok::<_, mpsc::Error>(())
//         });
//         t.await??;
//         Ok(&format!("Player {} authenticated.", self.player))
//     }

//     pub(crate) fn create_game(&self, game_title: &str) -> Result<&str, &str>  {
//         Ok(&format!("Player {} created game {}.", self.player, game_title))
//     }

//     pub(crate) fn build_deck(&self, suits: Vec<&str>, values: Vec<&str>, backside: &str) -> Result<&str, &str> {
//         let id = 0;
//     }
// }
