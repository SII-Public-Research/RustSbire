use std::time::Duration;

use eyre::WrapErr;
use linux_embedded_hal::{
    spidev::{SpidevOptions, SPI_MODE_3},
    Pin, Spidev,
};
use pscontroller_rs::{Device, PlayStationPort};
use rust_sbire::Component;
use tokio::{sync::mpsc::Sender, time::sleep};

use crate::{ControlMode, Velocity};

pub struct RemoteControl;

const SPI_DEV_PATH: &str = "/dev/spidev0.0";
const SPI_SPEED: u32 = 100_000;

impl Component<Sender<ControlMode>> for RemoteControl {
    type Error = eyre::Report;

    async fn run(tx_remote: Sender<ControlMode>) -> eyre::Result<()> {
        const POLL_INTERVAL: Duration = Duration::from_millis(20);

        // Initial data.
        let mut x = 0.;
        let mut y = 0.;
        let mut theta = 0.;
        let mut controlled_by_remote = false;
        let mut was_select_pressed = true;

        // Init.

        let mut spi = Spidev::open(SPI_DEV_PATH).wrap_err("Failed to open SPI device")?;
        spi.configure(
            &SpidevOptions::new()
                .bits_per_word(8)
                .max_speed_hz(SPI_SPEED)
                .mode(SPI_MODE_3)
                .build(),
        )
        .wrap_err("Failed to configure SPI device")?;
        let mut ps_port = PlayStationPort::new(spi, None::<Pin>);
        ps_port
            .enable_pressure()
            .wrap_err("Failed to enable DualShock2 mode")?;

        loop {
            sleep(POLL_INTERVAL).await;

            let device = match ps_port.read_input(None) {
                Err(err) => {
                    println!("[remote] Error reading controller: {err:?}"); // TODO
                    continue;
                }
                Ok(controller) => controller,
            };
            match device {
                Device::DualShock2(ds2) => {
                    let is_select_pressed = ds2.buttons.select();
                    // Toggle the control mode when the button is newly pressed.
                    if is_select_pressed && !was_select_pressed {
                        controlled_by_remote = !controlled_by_remote;
                        println!(
                            "[remote] Select pressed, remote control now {}abled",
                            if controlled_by_remote { "en" } else { "dis" }
                        );
                    }
                    was_select_pressed = is_select_pressed;

                    (x, y) = if ds2.buttons.up() {
                        (100., 0.)
                    } else if ds2.buttons.down() {
                        (-100., 0.)
                    } else if ds2.buttons.left() {
                        (0., 100.)
                    } else if ds2.buttons.right() {
                        (0., -100.)
                    } else {
                        (0., 0.)
                    };

                    theta = if ds2.buttons.square() {
                        100.
                    } else if ds2.buttons.circle() {
                        -100.
                    } else {
                        0.
                    };
                }
                device => {
                    println!(
                        "Unsupported controller type \"{}\", manual control disabled",
                        controller_name(&device),
                    );
                    controlled_by_remote = false;
                }
            }

            // On met tout ca dans le channel
            tx_remote
                .send(if controlled_by_remote {
                    ControlMode::Manual(Velocity { x, y, theta })
                } else {
                    ControlMode::Automatic
                })
                .await
                .wrap_err("Failure to send velocity")?;
        }
    }
}

fn controller_name(device: &Device) -> &'static str {
    match device {
        Device::None => "none",
        Device::Unknown => "unknown",
        Device::ConfigurationMode => "configuration mode",
        Device::Mouse(_) => "mouse",
        Device::Classic(_) => "classic",
        Device::AnalogJoystick(_) => "analog joystick",
        Device::DualShock(_) => "dualshock",
        Device::DualShock2(_) => "dualshock2",
        Device::GuitarHero(_) => "guitar hero",
        Device::JogCon(_) => "jogcon",
        Device::NegCon(_) => "negcon",
        Device::GunCon(_) => "guncon",
        Device::Baton(_) => "baton",
    }
}
