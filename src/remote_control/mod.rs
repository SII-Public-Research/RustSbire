use std::sync::mpsc::Sender;

use super::*;

pub struct RemoteControl {
    _quoi_mettre: u32,
}

type SenderRemoteMode = (Sender<Velocity>, Sender<Mode>);

impl Component<SenderRemoteMode> for RemoteControl {
    fn init() -> Self {
        println!("Remote control is initialised");
        RemoteControl { _quoi_mettre: 0 }
    }

    fn main_thread(self, (tx_remote, tx_mode): SenderRemoteMode) {
        println!("We are executing code inside the main function of the RemoteControl");
        let mut vel = Velocity {
            x: 0.,
            y: 0.,
            theta: 0.,
        };
        let mut mode = Mode {
            controlled_by_remote: false,
        };

        loop {
            thread::sleep(Duration::from_millis(1000));

            // Algorithmie
            vel.x += 1.;
            vel.y += 1.;
            vel.theta += 1.;

            // On met tout ca dans le channel
            tx_remote.send(vel).unwrap();

            mode.controlled_by_remote = !mode.controlled_by_remote;
            tx_mode.send(mode).unwrap();
        }
    }
}
