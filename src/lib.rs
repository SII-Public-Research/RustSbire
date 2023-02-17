
use std::thread;
use std::time::Duration;

pub mod motors;
pub mod key_control;


pub trait Component<A> {
    fn init() -> Self;
    fn main_thread(arg: A);
}
