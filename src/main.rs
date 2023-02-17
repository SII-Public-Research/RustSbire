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

use rust_sbire::{key_control::KeyboardControl, motors::Motors, Component, effet_hall::EffetHallAlgo};

use std::thread;



fn main() {
    println!("Coucou, connard !");

    /* HERE WILL BE ALL THE CHANNEL AND THEIR DESCRIPTIONS 
        | CHANNEL NAME       | SENDER        | RECEIVER     |
        ----------------------------------------------------
        | cmd_vel            | Control       | Motors       |
    */
    let (cmd_vel_tx, cmd_vel_rx) = mpsc::channel();
    let (cmd_vel_tx1, cmd_vel_rx1) = mpsc::channel();
    let (cmd_vel_tx2, cmd_vel_rx2) = mpsc::channel();

    let _my_components = (
        Motors::default(), 
        KeyboardControl::default(),
        EffetHallAlgo::default(),
    );

    let motor_task = thread::spawn(move || {
        Motors::main_thread(cmd_vel_rx);
    });
    let keyboard_task = thread::spawn(move || {
        KeyboardControl::main_thread(cmd_vel_tx);
    });
    let algo_hall_task = thread::spawn(move || {
        EffetHallAlgo::main_thread((cmd_vel_tx1, cmd_vel_rx2));
    });

    motor_task.join().unwrap();
    keyboard_task.join().unwrap();
    algo_hall_task.join().unwrap();
}
