use std::time::Duration;

use rust_sbire::Component;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::sleep,
};

use crate::{BFieldData, Position};

pub struct EffetHallAlgo;

type SenderReceiver = (Sender<Position>, Receiver<BFieldData>);
impl Component<SenderReceiver> for EffetHallAlgo {
    async fn run((tx, mut rx): SenderReceiver) {
        println!("We are executing code inside the main function of the EffetHallAlgo");
        let mut pos = Position { x: 0., y: 0. };

        loop {
            sleep(Duration::from_millis(1000)).await;

            let bfield = rx.try_recv();
            if bfield.is_ok() {
                println!(
                    " B Field x = {}, y = {}, z = {}",
                    bfield.unwrap().x,
                    bfield.unwrap().y,
                    bfield.unwrap().z
                );
            }
            // Algorithmie
            pos.x += 0.5;
            pos.y += 0.5;

            // On met tout ca dans le channel
            tx.send(pos).await.unwrap();
        }
    }
}
