use bevy::prelude::*;
use std::collections::HashMap;
use crate::shells::movement::*;

pub const GRAVITY: Vec2 = Vec2::new(0., -60.);

#[derive(Clone)]
struct AcceleratorFn (fn(thrust: &f32, velocity: &mut Velocity, d_time: &f32, burn_time: &Timer));

pub fn negative_gravity(thrust: &f32, velocity: &mut Velocity, d_time: &f32, burn_time: &Timer) {
    velocity.0 -= GRAVITY * *d_time;
}

pub fn randomize_accelerator(thrust: &f32, velocity: &mut Velocity, d_time: &f32, burn_time: &Timer) {
    let acceleration = *thrust
        * Vec2::new(
            1. - 2. * rand::random::<f32>(),
            1. - 2. * rand::random::<f32>(),
        );
    velocity.0 += acceleration;
}

pub fn exponential_accelerator(thrust: &f32, velocity: &mut Velocity, d_time: &f32, burn_time: &Timer) {
    let acceleration =
        *thrust * velocity.0.normalize() * (-(burn_time.elapsed().as_millis() as f32) / 500.).exp();
    // let acceleration = *thrust * velocity.0.normalize();
    velocity.0 += acceleration * *d_time;
}

#[derive(Component, Clone)]
pub struct Accelerator {
    pub accelerator: fn(thrust: &f32, velocity: &mut Velocity, d_time: &f32, burn_time: &Timer),
    pub burn_time: Timer,
    pub thrust: f32,
}

impl Accelerator {
    pub fn get_velocity(&self, velocity: &mut Velocity, d_time: &f32) {
        (self.accelerator)(&self.thrust, velocity, d_time, &self.burn_time);
    }
}

pub struct MappedAccelerators {
    accelerator_map: HashMap<&'static str, fn(thrust: &f32, velocity: &mut Velocity, d_time: &f32, burn_time: &Timer)>
}

impl MappedAccelerators {
    pub fn new() -> Self {
        let accelerator_map = HashMap::from([
            ("negative_gravity", negative_gravity as fn(thrust: &f32, velocity: &mut Velocity, d_time: &f32, burn_time: &Timer)),
            ("randomize_accelerator", randomize_accelerator as fn(thrust: &f32, velocity: &mut Velocity, d_time: &f32, burn_time: &Timer)),
            ("exponential_accelerator", exponential_accelerator as fn(thrust: &f32, velocity: &mut Velocity, d_time: &f32, burn_time: &Timer)),
        ]);
        Self {
            accelerator_map
        }
    }
}