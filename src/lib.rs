use std::thread;
use std::time::Duration;

pub mod effet_hall_algo;
pub mod effet_hall_data;
pub mod motors;
pub mod movement_algo;
pub mod remote_control;

/// Composant par défaut, A est le type de variable échangée entre threads
pub trait Component<A> {
    fn init() -> Self;
    fn main_thread(self, arg: A);
}

/// Données de champs magnétique
#[derive(Clone, Copy)]
pub struct BFieldData {
    x: f32,
    y: f32,
    z: f32,
}

/// Données de vitesse linéaires et angulaires
#[derive(Clone, Copy)]
pub struct Velocity {
    x: f32,     // m/s
    y: f32,     // m/s
    theta: f32, // rad/s
}

/// Données de position
#[derive(Clone, Copy)]
pub struct Position {
    x: f32, // m
    y: f32, // m
}

/// Mode
#[derive(Clone, Copy)]
pub struct Mode {
    controlled_by_remote: bool,
}
