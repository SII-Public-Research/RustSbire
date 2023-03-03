use std::time::Duration;

use rust_sbire::Component;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::sleep,
};

use crate::{Position, Velocity};

pub struct MovementAlgo;

type SenderReceiver = (Sender<Velocity>, Receiver<Position>);
impl Component<SenderReceiver> for MovementAlgo {
    async fn run((tx, mut rx): SenderReceiver) {
        println!("We are executing code inside the main function of the MovementAlgo");
        let mut vel = Velocity {
            x: 0.,
            y: 0.,
            theta: 0.,
        };

        loop {
            sleep(Duration::from_millis(1000)).await;

            let pos = rx.try_recv();
            if pos.is_ok() {
                println!(" Pos x = {}, y = {}", pos.unwrap().x, pos.unwrap().y);
            }
            // Algorithmie
            vel.x -= 1.;
            vel.y -= 0.2;
            vel.theta -= 0.01;

            // On met tout ca dans le channel
            tx.send(vel).await.unwrap();
        }
    }
}
