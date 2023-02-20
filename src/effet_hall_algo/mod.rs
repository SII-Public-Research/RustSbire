use std::sync::mpsc::{Receiver, Sender};

use super::*;

pub struct EffetHallAlgo {
    _quoi_mettre: u32,
}
type SenderReceiver = (Sender<Position>, Receiver<BFieldData>);

impl Component<SenderReceiver> for EffetHallAlgo {
    fn init() -> Self {
        println!("EffetHallAlgo is initialised");
        EffetHallAlgo { _quoi_mettre: 0 }
    }

    fn main_thread((tx, rx): SenderReceiver) {
        println!("We are executing code inside the main function of the EffetHallAlgo");
        let mut pos = Position { x: 0., y: 0. };

        loop {
            thread::sleep(Duration::from_millis(1000));

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
            tx.send(pos).unwrap();
        }
    }
}
