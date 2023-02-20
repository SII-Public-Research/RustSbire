use std::sync::mpsc::{Receiver, Sender};

use super::*;

pub struct MovementAlgo {
    _quoi_mettre: u32,
}
type SenderReceiver = (Sender<(i32, i32, i32)>, Receiver<(i32, i32, i32)>);

impl Component<SenderReceiver> for MovementAlgo {
    fn init() -> Self {
        println!("keyboard control is initialised");
        MovementAlgo { _quoi_mettre: 0 }
    }

    fn main_thread((tx, rx): SenderReceiver) {
        println!("We are executing code inside the main function of the MovementAlgo");
        let (mut x, mut y, mut theta) = (0, 0, 0);
        // println!("Starting MovementAlgo thread");
        loop {
            thread::sleep(Duration::from_millis(1000));

            // Algorithmie
            x += 1;
            y += 1;
            theta += 1;

            // On met tout ca dans le channel
            tx.send((x, y, theta)).unwrap();
        }
    }
}

impl Default for MovementAlgo {
    fn default() -> Self {
        Self::init()
    }
}
