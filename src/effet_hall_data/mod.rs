use std::sync::mpsc::Sender;

use super::*;

pub struct EffetHallData {
    _quoi_mettre: u32,
}

impl Component<Sender<BFieldData>> for EffetHallData {
    fn init() -> Self {
        println!("EffetHallData is initialised");
        EffetHallData { _quoi_mettre: 0 }
    }

    fn main_thread(self, tx: Sender<BFieldData>) {
        println!("We are executing code inside the main function of the EffetHallData");
        let mut data = BFieldData {
            x: 0.,
            y: 0.,
            z: 0.,
        };

        loop {
            thread::sleep(Duration::from_millis(1000));

            // Algorithmie
            data.x += 1.;
            data.y += 1.;
            data.z += 1.;

            // On met tout ca dans le channel
            tx.send(data).unwrap();
        }
    }
}
