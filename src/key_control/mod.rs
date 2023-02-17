use std::sync::mpsc::Sender;

use super::*;

pub struct KeyboardControl {
    _quoi_mettre: u32,
}

impl KeyboardControl {
    pub fn new() -> Self {
        println!("keyboard control is initialised");
        KeyboardControl {
            _quoi_mettre: 0,
        }
    }

    pub fn main(tx: Sender<(i32, i32, i32)>) {
        println!("We are executing code inside the main function of the KeyboardControl");
        let (mut x, mut y, mut theta) = (0, 0, 0);
        // println!("Starting KeyboardControl thread");
        loop {
            thread::sleep(Duration::from_millis(1000));

            // Algorithmie 
            x += 1;
            y += 1;
            theta += 1;

            // On met tout ca dans le channel
            tx.send((x,y,theta)).unwrap();
        }
    }
}

impl Default for KeyboardControl {
    fn default() -> Self {
        Self::new()
    }
}