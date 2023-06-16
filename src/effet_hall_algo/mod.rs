use std::{borrow::Borrow, ops::Deref};

use eyre::Context;
use rust_sbire::Component;
use tokio::sync::watch::{Receiver, Sender};

use crate::{BFieldData, Position};

pub struct EffetHallAlgo;

type SenderReceiver = (Sender<Option<Position>>, Receiver<BFieldData>);
impl Component<SenderReceiver> for EffetHallAlgo {
    type Error = eyre::Report;

    async fn run((tx, mut rx): SenderReceiver) -> eyre::Result<()> {
        // Seuil (regarder quelles valeurs mettre de manière expérimentale).
        const SEUIL_3DHALL: f64 = 0.1; // mT

        // Coefficients à régler, pour estimer la distance à l'aimant (CF doc Melexis).
        const MAX_R: f64 = 3.0;
        const COEFF_C: f64 = MAX_R / 1.571; //   = max_r / lim_(x->inf)_atan(x)
        const COEFF_K: f64 = 1.0;

        // Distance estimée à partir de la quelle on considère que le robot est bien positionné.
        const MIN_R: f64 = COEFF_C * 0.2;

        println!("We are executing code inside the main function of the EffetHallAlgo");

        loop {
            let Ok(()) = rx.changed().await else {
                println!("[algo] Hall's channel was closed, ending...");
                return Ok(());
            };

            let &BFieldData { x, y, z } = rx.borrow().deref();
            let xy = crate::norm2d(x, y);
            let velocity = if xy > SEUIL_3DHALL {
                let theta = y.atan2(x);
                let r = if z < 0. {
                    match COEFF_C * (xy / (COEFF_K * -z)).atan() {
                        r if r < MIN_R => 0.,
                        r => r,
                    }
                } else {
                    MAX_R
                };

                Some(Position {
                    x: r * theta.cos(),
                    y: r * theta.sin(),
                    theta,
                })
            } else {
                // The magnet is too far.
                None
            };

            // On met tout ca dans le channel.
            println!("[algo] {velocity:?}");
            tx.send(velocity)
                .wrap_err("Failure to send movement algo")?;
        }
    }
}
