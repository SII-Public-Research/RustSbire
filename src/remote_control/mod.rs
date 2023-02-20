use std::sync::mpsc::Sender;

use super::*;

pub struct RemoteControl {
    _quoi_mettre: u32,
}

type SenderRemoteMode = (Sender<(i32, i32, i32)>, Sender<(i32, i32, i32)>);

impl Component<SenderRemoteMode> for RemoteControl {
    fn init() -> Self {
        println!("keyboard control is initialised");
        RemoteControl { _quoi_mettre: 0 }
    }

    fn main_thread((tx_remote, tx_mode): SenderRemoteMode) {
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
            tx_remote.send((x, y, theta)).unwrap();
        }
    }
}

impl Default for RemoteControl {
    fn default() -> Self {
        Self::init()
    }
}
