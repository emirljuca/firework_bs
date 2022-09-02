use bevy::prelude::*;
use crate::shells::movement::*;

pub const GRAVITY: Vec2 = Vec2::new(0., -60.);

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