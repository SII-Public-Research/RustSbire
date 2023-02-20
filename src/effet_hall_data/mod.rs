use std::sync::mpsc::Sender;

use super::*;

pub struct EffetHallData {
    _quoi_mettre: u32,
}

impl Component<Sender<(i32, i32, i32)>> for EffetHallData {
    fn init() -> Self {
        println!("keyboard control is initialised");
        EffetHallData { _quoi_mettre: 0 }
    }

    fn main_thread(tx: Sender<(i32, i32, i32)>) {
        println!("We are executing code inside the main function of the EffetHallData");
        let (mut x, mut y, mut theta) = (0, 0, 0);
        // println!("Starting EffetHallData thread");
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

impl Default for EffetHallData {
    fn default() -> Self {
        Self::init()
    }
}
