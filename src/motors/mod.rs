use std::time::Duration;

use rust_sbire::Component;
use tokio::{
    sync::mpsc::{error::TryRecvError, Receiver},
    time::{sleep, timeout, Instant},
};

use crate::{ControlMode, Velocity};

pub struct Motors;

struct MotorData {
    left: u32,
    right: u32,
}

type ReceiversRemoteAlgoMode = (Receiver<ControlMode>, Receiver<Velocity>);
impl Component<ReceiversRemoteAlgoMode> for Motors {
    type Error = eyre::Report;

    async fn run((mut rx_remote, mut rx_algo): ReceiversRemoteAlgoMode) -> eyre::Result<()> {
        const CMD_TIMEOUT: Duration = Duration::from_millis(100);

        let mut data = MotorData { left: 0, right: 0 };
        println!("We are executing code inside the main function of the Motors");

        loop {
            // Make sure to always receive from both channels, to prevent the buffers getting full.
            let remote_cmd = timeout(CMD_TIMEOUT, rx_remote.recv()).await;
            let algo_cmd = rx_algo.try_recv();

            let velocity = match (remote_cmd, algo_cmd) {
                (Ok(None), _) => {
                    println!("[motors] Remote's channel was closed, ending...");
                    return Ok(());
                }
                (_, Err(TryRecvError::Disconnected)) => {
                    println!("[motors] Algo's channel was closed, ending...");
                    return Ok(());
                }
                // We must at least receive the remote's command, since it indicates whether we should accept the algo's.
                (Err(_), _) | (Ok(Some(ControlMode::Automatic)), Err(TryRecvError::Empty)) => {
                    println!("[motors] Command reception timed out, stopping motors");
                    // TODO
                    continue;
                }
                // All's safe! Now we can pick one of the two.
                (Ok(Some(ControlMode::Manual(cmd))), _) => cmd,
                (Ok(Some(ControlMode::Automatic)), Ok(cmd)) => cmd,
            };

            println!(
                "[motors] Vx = {}, Vy = {}, Vtheta = {}",
                velocity.x, velocity.y, velocity.theta
            );
            //let algo_vel = rx_algo.recv().unwrap();
            //let mode = rx_mode.recv().unwrap();

            //println!(" Remote Vx = {}, Vy = {}, Vtheta = {}", remote_vel.x, remote_vel.y, remote_vel.theta);
            //println!(" Algo Vx = {}, Vy = {}, Vtheta = {}", algo_vel.x, algo_vel.y, algo_vel.theta);
            //println!(" Mode = {}", mode.controlled_by_remote);
        }
    }
}
