use std::time::Duration;

use eyre::Context;
use rust_sbire::Component;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::sleep,
};

use crate::{BFieldData, Position};

pub struct EffetHallAlgo;

type SenderReceiver = (Sender<Position>, Receiver<BFieldData>);
impl Component<SenderReceiver> for EffetHallAlgo {
    type Error = eyre::Report;

    async fn run((tx, mut rx): SenderReceiver) -> eyre::Result<()> {
        println!("We are executing code inside the main function of the EffetHallAlgo");
        let mut pos = Position { x: 0., y: 0. };

        loop {
            sleep(Duration::from_millis(1000)).await;

            let bfield = rx.try_recv();
            if let Ok(BFieldData { x, y, z }) = bfield {
                // println!(" B Field x = {x}, y = {y}, z = {z}");
            }
            // Algorithmie
            pos.x += 0.5;
            pos.y += 0.5;

            // On met tout ca dans le channel
            tx.send(pos)
                .await
                .wrap_err("Failed to send Hall effect algo")?;
        }
    }
}
