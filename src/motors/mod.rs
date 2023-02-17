use std::sync::mpsc::Receiver;

use super::*;

pub struct Motors {
    _left: u32,
    _right: u32,
}

impl Motors {
    pub fn new() -> Self {
        print!("Motors are initialized !");
        Motors {
            _left: 0,
            _right: 0,
        }
    }

    pub fn main(rx: Receiver<(i32, i32, i32)>) {
        println!("We are executing code inside the main function of the Motors");
        thread::spawn(move || {
            // println!("Starting Motors thread");
            loop {
                // thread::sleep(Duration::from_millis(1000));

                let (x, y, theta) = rx.recv().unwrap();
                let val = String::from("coucou");
                println!(" Received Vx = {}, Vy = {}, Vtheta = {}", x, y, theta);
            }
        });
    }
}
