use std::sync::mpsc::{Receiver, Sender};

use super::*;

pub struct MovementAlgo {
    _quoi_mettre: u32,
}
type SenderReceiver = (Sender<Velocity>, Receiver<Position>);

impl Component<SenderReceiver> for MovementAlgo {
    fn init() -> Self {
        println!("MovementAlgo is initialised");
        MovementAlgo { _quoi_mettre: 0 }
    }

    fn main_thread((tx, rx): SenderReceiver) {
        println!("We are executing code inside the main function of the MovementAlgo");
        //let (mut x, mut y, mut theta) = (0, 0, 0);
        let mut vel = Velocity {
            x: 0.,
            y: 0.,
            theta: 0.,
        };
        // println!("Starting RemoteControl thread");
        loop {
            thread::sleep(Duration::from_millis(1000));

            // Algorithmie
            vel.x -= 1.;
            vel.y -= 0.2;
            vel.theta -= 0.01;

            // On met tout ca dans le channel
            tx.send(vel).unwrap();
        }
    }
}

impl Default for MovementAlgo {
    fn default() -> Self {
        Self::init()
    }
}
