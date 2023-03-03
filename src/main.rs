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

use tokio::{join, sync::mpsc};

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
#[derive(Debug, Clone, Copy)]
pub struct BFieldData {
    x: f32,
    y: f32,
    z: f32,
}

/// Données de vitesse linéaires et angulaires
#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    x: f32,     // m/s
    y: f32,     // m/s
    theta: f32, // rad/s
}

/// Données de position
#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: f32, // m
    y: f32, // m
}

/// Mode
#[derive(Debug, Clone, Copy)]
pub struct Mode {
    controlled_by_remote: bool,
}

#[tokio::main]
async fn main() {
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
    let (remote_cmd_vel_tx, remote_cmd_vel_rx) = mpsc::channel(4);
    let (algo_cmd_vel_tx, algo_cmd_vel_rx) = mpsc::channel(4);
    let (position_tx, position_rx) = mpsc::channel(4);
    let (hall_data_tx, hall_data_rx) = mpsc::channel(4);
    let (mode_tx, mode_rx) = mpsc::channel(4);

    let motor_task = Motors::run((remote_cmd_vel_rx, algo_cmd_vel_rx, mode_rx));

    let remote_control_task = RemoteControl::run((remote_cmd_vel_tx, mode_tx));

    let algo_move_task = MovementAlgo::run((algo_cmd_vel_tx, position_rx));

    let algo_hall_task = EffetHallAlgo::run((position_tx, hall_data_rx));

    let data_hall_task = EffetHallData::run(hall_data_tx);

    join!(
        motor_task,
        remote_control_task,
        algo_move_task,
        algo_hall_task,
        data_hall_task
    );
}
