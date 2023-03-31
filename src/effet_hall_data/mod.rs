use std::{ops::Sub, path::Path, time::Duration};

use eyre::{eyre, Context};
use linux_embedded_hal::{i2cdev::linux::LinuxI2CError, Delay, I2cdev};
use rust_sbire::Component;
use tlv493d_a1b6::{Bfield, Mode, Tlv493d, ADDRESS_BASE_1};
use tokio::{sync::mpsc::Sender, time::sleep};

use crate::BFieldData;

pub struct EffetHallData;

type EffetHallParams = (Sender<BFieldData>, Delay);
impl Component<EffetHallParams> for EffetHallData {
    type Error = eyre::Report;

    // TODO: actually blocking the thread is not great, can it be replaced with an `await`?
    async fn run((tx, mut delay): EffetHallParams) -> eyre::Result<()> {
        const SENSOR_I2C_ADDR: u8 = ADDRESS_BASE_1;

        let mut sensor = Sensor::new("/dev/i2c-1", SENSOR_I2C_ADDR)
            .wrap_err("Failed to open magnetic sensor")?;
        let resting_value = sensor
            .get_mean(600, &mut delay)
            .wrap_err("Failed to read sensor resting value")?;

        loop {
            sleep(Duration::from_millis(10)).await; // TODO: adjust this, or remove it entirely

            let data = sensor
                .get_mean(150, &mut delay)
                .wrap_err("Failed to read sensor")?
                - resting_value;

            // On met tout ca dans le channel
            tx.send(data)
                .await
                .wrap_err("Failure to send Hall effect data")?;
        }
    }
}

// The inner is neither `Debug` nor `Clone`. --'
struct Sensor(Tlv493d<I2cdev, LinuxI2CError>);

impl Sensor {
    fn new<P: AsRef<Path>>(path: P, addr: u8) -> eyre::Result<Self> {
        let i2c = I2cdev::new(path.as_ref()).wrap_err("Failed to open I2C bus")?;
        let tlv = Tlv493d::new(i2c, addr, &Mode::Master)
            .map_err(|err| eyre!("Error opening TLV493D sensor: {err}"))?;
        Ok(Self(tlv))
    }

    fn get_mean(&mut self, nb_samples: u16, delay: &mut Delay) -> eyre::Result<BFieldData> {
        loop {
            break match self.0.get_b_mean::<u16, _>(nb_samples, delay) {
                Ok(b) => Ok(b.into()),
                Err(tlv493d_a1b6::Error::AdcLockup) => {
                    // TODO: something equivalent used to be done, but cannot be implemented as-is rn. Is it necessary?
                    // self.0 = Tlv493d::new(self.0.i2c, self.0.addr, &Mode::Master)?;
                    continue;
                }
                Err(err) => Err(err.into()),
            };
        }
    }
}

impl From<Bfield> for BFieldData {
    fn from(value: Bfield) -> Self {
        Self {
            x: value.bx.into(),
            y: value.by.into(),
            z: value.bz.into(),
        }
    }
}

impl Sub<Self> for BFieldData {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
