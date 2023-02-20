use std::sync::mpsc::Receiver;

use super::*;

pub struct Motors {
    _left: u32,
    _right: u32,
}

type ReceiversRemoteAlgoMode = (
    Receiver<(i32, i32, i32)>,
    Receiver<(i32, i32, i32)>,
    Receiver<(i32, i32, i32)>,
);

impl Component<ReceiversRemoteAlgoMode> for Motors {
    fn init() -> Self {
        print!("Motors are initialized !");
        Motors {
            _left: 0,
            _right: 0,
        }
    }

    fn main_thread((rx_remote, rx_algo, rx_mode): ReceiversRemoteAlgoMode) {
        println!("We are executing code inside the main function of the Motors");
        // println!("Starting Motors thread");
        loop {
            // thread::sleep(Duration::from_millis(1000));

            let (x, y, theta) = rx_remote.recv().unwrap();
            let _val = String::from("coucou");
            println!(" Received Vx = {}, Vy = {}, Vtheta = {}", x, y, theta);
        }
    }
}

impl Default for Motors {
    fn default() -> Self {
        Self::init()
    }
}
