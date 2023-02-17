
use std::thread;
use std::time::Duration;

pub mod motors;
pub mod key_control;


pub trait Component {
    fn init();
    fn main_thread();
}
