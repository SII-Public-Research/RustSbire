//! PRESENTATION OF THE RUST SBIRE !
//! ---------------------------------
//! This is an attempt to create a subsitute to ROS using only RUST programming language
//! The idea is to first define a simple robot architecture and organize the differents packages using threads.
//! Each thread will represent a package of functionnalities,
//! and data will be exchanged between threads using Channels.
//!
//! This project has two objectives :
//!     - Upgrade our level on RUST (see chapters 13, 16, AND ??)
//!     - Looking at a solution to do robots without using ROS. WIth this project, we could compare performances between ROS and full RUST robot.
//!
//! FIRST PART : ARCHITECTURE
//! When I am thinking about architecture, I am thinking both to :
//!     - What kind of functionnalities we should implement on this first version
//!     - How to structure the code
//!
//! FUNCTIONNALITIES :
//! As it should be simple at the beginning, I am thinking of a simple robot that the user could control via keyboard.
//! It needs basic motor-control part and a command.
//!
//! CODE ORGANIZATION :
//! I would like to organize the code using threads. One thread will represent a fonctionnality.
//! For this first example, I only need two thread :
//!     - A control thread : It should take keybord input and transform it into a command
//!     - A motor control thread : it will take a command and use it to move the motors.
//! Between these two threads, a simple channel will be exchanged, containing the command.
//!
//!
//!

#![feature(async_fn_in_trait)]

use eyre::WrapErr;
use tokio::{
    sync::{mpsc, watch},
    task, try_join,
};

use {
    effet_hall_algo::EffetHallAlgo, effet_hall_data::EffetHallData, motors::Motors,
    movement_algo::MovementAlgo, remote_control::RemoteControl, rust_sbire::Component,
};

pub mod effet_hall_algo;
pub mod effet_hall_data;
pub mod motors;
pub mod movement_algo;
pub mod remote_control;

/// Données de champs magnétique
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BFieldData {
    x: f64,
    y: f64,
    z: f64,
}

/// Données de vitesse linéaires et angulaires
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Velocity {
    /// In m/s
    x: f64,
    /// In m/s
    y: f64,
    /// In rad/s
    theta: f64,
}

/// Données de position
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Position {
    x: f64,     // m
    y: f64,     // m
    theta: f64, // rad
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ControlMode {
    #[default]
    Automatic,
    Manual(Velocity),
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    println!("Coucou, petit sbire !");

    /* HERE WILL BE ALL THE CHANNEL AND THEIR DESCRIPTIONS
        | CHANNEL NAME       | SENDER        | RECEIVER      |
        ------------------------------------------------------
        | remote_cmd_vel     | RemoteControl | Motors        |
        | algo_cmd_vel       | MovementAlgo  | Motors        |
        | position           | EffetHallAlgo | MovementAlgo  |
        | hall_data          | EffetHallData | EffetHallAlgo |
        | mode               | RemoteControl | Motors        |
    */
    let (remote_cmd_tx, remote_cmd_rx) = watch::channel(Default::default());
    let (hall_data_tx, hall_data_rx) = watch::channel(Default::default());
    let (position_tx, position_rx) = watch::channel(Default::default());
    let (algo_cmd_vel_tx, algo_cmd_vel_rx) = watch::channel(Default::default());

    let data_hall_task =
        task::spawn_blocking(move || EffetHallData::run((hall_data_tx, linux_embedded_hal::Delay)));

    try_join!(
        RemoteControl::run(remote_cmd_tx),
        async {
            data_hall_task
                .await
                .wrap_err("Error joining data_hall task")
                .and_then(|result| result)
        },
        EffetHallAlgo::run((position_tx, hall_data_rx)),
        MovementAlgo::run((algo_cmd_vel_tx, position_rx)),
        Motors::run((remote_cmd_rx, algo_cmd_vel_rx)),
    )?;
    //data_hall_task
    //    .await
    //    .wrap_err("Error joining data_hall task")?
    //    .wrap_err("Error in data_hall task")?;
    Ok(())
}

struct ChannelDrain;

impl<T> Component<mpsc::Receiver<T>> for ChannelDrain {
    type Error = eyre::Report;

    async fn run(mut rx: mpsc::Receiver<T>) -> Result<(), Self::Error> {
        loop {
            drop(rx.recv().await);
        }
    }
}

fn norm2d(x: f64, y: f64) -> f64 {
    (x.powi(2) + y.powi(2)).sqrt()
}
