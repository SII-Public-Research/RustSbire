use std::thread;
use std::time::Duration;

pub mod effet_hall_algo;
pub mod effet_hall_data;
pub mod motors;
pub mod movement_algo;
pub mod remote_control;

pub trait Component<A> {
    fn init() -> Self;
    fn main_thread(arg: A);
}
