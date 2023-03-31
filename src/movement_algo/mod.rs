use eyre::Context;
use rust_sbire::Component;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{norm2d, Position, Velocity};

pub struct MovementAlgo;

const MAX_LIN_VEL: f64 = 100.;

type SenderReceiver = (Sender<Velocity>, Receiver<Option<Position>>);
impl Component<SenderReceiver> for MovementAlgo {
    type Error = eyre::Report;

    async fn run((tx, mut rx): SenderReceiver) -> eyre::Result<()> {
        println!("We are executing code inside the main function of the MovementAlgo");

        let mut target_x_lin_vel = 0.;
        let mut target_y_lin_vel = 0.;
        let mut target_z_lin_vel = 0.; // TODO: I'm pretty sure this name is wrong.
        loop {
            let Some(recv) = rx.recv().await else {
                println!("[movement] Hall algo channel closed, ending...");
                return Ok(());
            };

            if let Some(Position { x, y, theta }) = recv {
                let angular_diff = find_angular_diff(theta);
                target_x_lin_vel = x / (1. + 3. * angular_diff.abs());
                target_y_lin_vel = y / (1. + 3. * angular_diff.abs());

                if x == 0. && y != 0. {
                    target_z_lin_vel = -3.18 * angular_diff;
                }
            }

            check_linear_limit_vel(&mut target_x_lin_vel, &mut target_y_lin_vel);

            // On met tout ca dans le channel
            tx.send(Velocity {
                x: target_x_lin_vel,
                y: target_y_lin_vel,
                theta: target_z_lin_vel,
            })
            .await
            .wrap_err("Failure to send movement algo")?;
        }
    }
}

fn find_angular_diff(theta: f64) -> f64 {
    let diff_1 = 1.571 - theta;
    let diff_2 = -1.571 - theta;

    if diff_1.abs() < diff_2.abs() {
        diff_1
    } else {
        diff_2
    }
}

fn check_linear_limit_vel(vx: &mut f64, vy: &mut f64) {
    let norm_vel = norm2d(*vx, *vy);
    let coef = norm_vel / MAX_LIN_VEL;
    if coef > 1.0 {
        *vx /= coef;
        *vy /= coef;
    }
}
