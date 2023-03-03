use std::time::Duration;

use rust_sbire::Component;
use tokio::{sync::mpsc::Sender, time::sleep};

use crate::{Mode, Velocity};

pub struct RemoteControl;

type SenderRemoteMode = (Sender<Velocity>, Sender<Mode>);
impl Component<SenderRemoteMode> for RemoteControl {
    async fn run((tx_remote, tx_mode): SenderRemoteMode) {
        println!("Remote control is initialised");
        let mut vel = Velocity {
            x: 0.,
            y: 0.,
            theta: 0.,
        };
        let mut mode = Mode {
            controlled_by_remote: false,
        };

        loop {
            sleep(Duration::from_millis(1000)).await;

            // Algorithmie
            vel.x += 1.;
            vel.y += 1.;
            vel.theta += 1.;

            // On met tout ca dans le channel
            tx_remote.send(vel).await.unwrap();

            mode.controlled_by_remote = !mode.controlled_by_remote;
            tx_mode.send(mode).await.unwrap();
        }
    }
}
