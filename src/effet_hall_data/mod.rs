use std::time::Duration;

use eyre::Context;
use rust_sbire::Component;
use tokio::{sync::mpsc::Sender, time::sleep};

use crate::BFieldData;

pub struct EffetHallData;

impl Component<Sender<BFieldData>> for EffetHallData {
    type Error = eyre::Report;

    async fn run(tx: Sender<BFieldData>) -> eyre::Result<()> {
        println!("We are executing code inside the main function of the EffetHallData");
        let mut data = BFieldData {
            x: 0.,
            y: 0.,
            z: 0.,
        };

        loop {
            sleep(Duration::from_millis(1000)).await;

            // Algorithmie
            data.x += 1.;
            data.y += 1.;
            data.z += 1.;

            // On met tout ca dans le channel
            tx.send(data)
                .await
                .wrap_err("Failure to send Hall effect data")?;
        }
    }
}
