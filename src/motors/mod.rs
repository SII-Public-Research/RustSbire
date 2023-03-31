use std::time::Duration;

use eyre::Context;
use linux_embedded_hal::I2cdev;
use rust_sbire::Component;
use sna41_motorshield::MotorShield;
use tokio::{
    sync::mpsc::{error::TryRecvError, Receiver},
    time::timeout,
};

use crate::{ControlMode, Velocity};

pub struct Motors;

type ReceiversRemoteAlgoMode = (Receiver<ControlMode>, Receiver<Velocity>);
impl Component<ReceiversRemoteAlgoMode> for Motors {
    type Error = eyre::Report;

    async fn run((mut rx_remote, mut rx_algo): ReceiversRemoteAlgoMode) -> eyre::Result<()> {
        const CMD_TIMEOUT: Duration = Duration::from_millis(100);

        let mut motor_shield =
            MotorShield::new(I2cdev::new("/dev/i2c-1").wrap_err("Failed to open I2C device")?)
                .expect("Failed to create MotorShield object"); // TODO: `expect` bad
        let mut set_motors = |commands: MotorCommands| {
            motor_shield
                .set_all_motors(commands.raw())
                .expect("Failed to set motors power") // TODO: `expect` bad
        };

        // Reset motors.
        set_motors(Default::default());

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
                (Ok(Some(ControlMode::Manual(cmd))), _) => cmd, // A manual command overrides the algorithm.
                (Ok(Some(ControlMode::Automatic)), Ok(cmd)) => cmd,
            };

            println!(
                "[motors] Vx = {}, Vy = {}, Vtheta = {}",
                velocity.x, velocity.y, velocity.theta
            );

            set_motors(MotorCommands::from(&velocity));
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct MotorCommands {
    front_left: f64,
    front_right: f64,
    back_left: f64,
    back_right: f64,
}

impl MotorCommands {
    fn new(
        mut front_left: f64,
        mut front_right: f64,
        mut back_left: f64,
        mut back_right: f64,
        max_rad: f64,
    ) -> Self {
        macro_rules! enforce_max {
            ($who:ident) => {
                let coef = $who.abs() / max_rad;
                if coef > 1.0 {
                    front_left /= coef;
                    front_right /= coef;
                    back_left /= coef;
                    back_right /= coef;
                }
            };
        }
        enforce_max!(front_left);
        enforce_max!(front_right);
        enforce_max!(back_left);
        enforce_max!(back_right);

        Self {
            front_left,
            front_right,
            back_left,
            back_right,
        }
    }

    fn raw(&self) -> [f32; 4] {
        [
            self.front_left as f32,
            self.front_right as f32,
            self.back_left as f32,
            self.back_right as f32,
        ]
    }
}

impl From<&Velocity> for MotorCommands {
    fn from(speed: &Velocity) -> Self {
        const WHEEL_RADIUS: f64 = 0.028; // en m
        const WHEEL_SEPARATION_X: f64 = 0.135 / 2.0; // en m
        const WHEEL_SEPARATION_Y: f64 = 0.146 / 2.0; // en m

        const MAX_RPM: f64 = 950.0; // nombre de rotations max d'une roue en une minute
        const RAD_TO_RPM: f64 = 9.55; // constante de conversion : 60 / (2 * pi)

        const MAX_RAD: f64 = MAX_RPM / RAD_TO_RPM;

        // V_x,max = R/4*(motors.front_left + motors.front_right  + motors.back_left + motors.back_right)
        // V_y,max = R/4*(-motors.front_left + motors.front_right  + motors.back_left - motors.back_right)
        // W_z,max = R/(4*(L+l)) * (-motors.front_left + motors.front_right  - motors.back_left + motors.back_right)

        let dx = speed.x;
        let dy = speed.y;
        let dtheta = speed.theta;

        Self::new(
            (1.0 / WHEEL_RADIUS) * (dx - dy - (WHEEL_SEPARATION_X + WHEEL_SEPARATION_Y) * dtheta), // en rad/s
            (1.0 / WHEEL_RADIUS) * (dx + dy + (WHEEL_SEPARATION_X + WHEEL_SEPARATION_Y) * dtheta), // en rad/s
            (1.0 / WHEEL_RADIUS) * (dx + dy - (WHEEL_SEPARATION_X + WHEEL_SEPARATION_Y) * dtheta), // en rad/s
            (1.0 / WHEEL_RADIUS) * (dx - dy + (WHEEL_SEPARATION_X + WHEEL_SEPARATION_Y) * dtheta), // en rad/s
            MAX_RAD,
        )
    }
}
