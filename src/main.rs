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
use std::sync::mpsc;

use RustSbire::key_control::KeyboardControl;
use RustSbire::motors::Motors;



fn main() {
    println!("Coucou, connard !");

    /* HERE WILL BE ALL THE CHANNEL AND THEIR DESCRIPTIONS 
        | CHANNEL NAME       | SENDER        | RECEIVER     |
        ----------------------------------------------------
        | cmd_vel            | Control       | Motors       |
    */
    let (cmd_vel_tx, cmd_vel_rx) = mpsc::channel();


    // let m_motors = Motors::new();
    // let m_keyboard = KeyboardControl::new();

    let _my_components = (Motors::new(), 
                                                            KeyboardControl::new());

    

    let _motor_task = Motors::main(cmd_vel_rx);
    let _keyboard_task = KeyboardControl::main(cmd_vel_tx);

    // join!(motor_task, keyboard_task);

    loop {}

    // thread::spawn(move || {
    //     println!("Starting keyControl thread");
    //     loop {
    //         thread::sleep(Duration::from_millis(1000));
    //         let val = String::from("coucou");
    //         cmd_vel_tx.send(val).unwrap();
    //     }
    // });









}
