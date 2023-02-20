use std::sync::mpsc::Receiver;

use super::*;

pub struct Motors {
    _left: u32,
    _right: u32,
}

type ReceiversRemoteAlgoMode = (Receiver<Velocity>, Receiver<Velocity>, Receiver<Mode>);

impl Component<ReceiversRemoteAlgoMode> for Motors {
    fn init() -> Self {
        println!("Motors are initialized !");
        Motors {
            _left: 0,
            _right: 0,
        }
    }

    fn main_thread(self, (rx_remote, rx_algo, rx_mode): ReceiversRemoteAlgoMode) {
        println!("We are executing code inside the main function of the Motors");

        loop {
            thread::sleep(Duration::from_millis(1000));

            let remote_vel = rx_remote.try_recv();
            if remote_vel.is_ok() {
                println!(
                    " Remote Vx = {}, Vy = {}, Vtheta = {}",
                    remote_vel.unwrap().x,
                    remote_vel.unwrap().y,
                    remote_vel.unwrap().theta
                );
            }
            let algo_vel = rx_algo.try_recv();
            if algo_vel.is_ok() {
                println!(
                    " Algo Vx = {}, Vy = {}, Vtheta = {}",
                    algo_vel.unwrap().x,
                    algo_vel.unwrap().y,
                    algo_vel.unwrap().theta
                );
            }
            let mode = rx_mode.try_recv();
            if mode.is_ok() {
                println!(" Mode = {}", mode.unwrap().controlled_by_remote);
            }
            //let algo_vel = rx_algo.recv().unwrap();
            //let mode = rx_mode.recv().unwrap();

            //println!(" Remote Vx = {}, Vy = {}, Vtheta = {}", remote_vel.x, remote_vel.y, remote_vel.theta);
            //println!(" Algo Vx = {}, Vy = {}, Vtheta = {}", algo_vel.x, algo_vel.y, algo_vel.theta);
            //println!(" Mode = {}", mode.controlled_by_remote);
        }
    }
}
