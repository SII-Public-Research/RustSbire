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
//use std::future::join;
use std::sync::mpsc;

use rust_sbire::{
    effet_hall_algo::EffetHallAlgo, effet_hall_data::EffetHallData, motors::Motors,
    movement_algo::MovementAlgo, remote_control::RemoteControl, Component,
};

use std::thread;

fn main() {
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
    let (remote_cmd_vel_tx, remote_cmd_vel_rx) = mpsc::channel();
    let (algo_cmd_vel_tx, algo_cmd_vel_rx) = mpsc::channel();
    let (position_tx, position_rx) = mpsc::channel();
    let (hall_data_tx, hall_data_rx) = mpsc::channel();
    let (mode_tx, mode_rx) = mpsc::channel();

    let _my_components = (
        Motors::init(),
        RemoteControl::init(),
        MovementAlgo::init(),
        EffetHallAlgo::init(),
        EffetHallData::init(),
    );

    let motor_task = thread::spawn(move || {
        Motors::main_thread((remote_cmd_vel_rx, algo_cmd_vel_rx, mode_rx));
    });
    let remote_control_task = thread::spawn(move || {
        RemoteControl::main_thread((remote_cmd_vel_tx, mode_tx));
    });
    let algo_move_task = thread::spawn(move || {
        MovementAlgo::main_thread((algo_cmd_vel_tx, position_rx));
    });
    let algo_hall_task = thread::spawn(move || {
        EffetHallAlgo::main_thread((position_tx, hall_data_rx));
    });
    let data_hall_task = thread::spawn(move || {
        EffetHallData::main_thread(hall_data_tx);
    });

    motor_task.join().unwrap();
    remote_control_task.join().unwrap();
    algo_move_task.join().unwrap();
    algo_hall_task.join().unwrap();
    data_hall_task.join().unwrap();
}
