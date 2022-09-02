use std::time::Duration;

use bevy::prelude::*;
use crate::shells::movement::*;
use crate::shells::accelerator::*;

#[derive(Component, Clone)]
pub struct Projectile {
    pub velocity: Velocity,
    pub life: Lifetime,
    pub image: Handle<Image>,
    pub color: Option<String>,
    pub accelerator: Option<Accelerator>,
}

impl Projectile {
    pub fn accelerate(&mut self, duration: Duration) {
        self.velocity.0 += GRAVITY * duration.as_secs_f32();
        if let Some(accelerator) = &mut self.accelerator {
            if !accelerator.burn_time.finished() {
                // Tick forward burn time of propultion.
                accelerator.burn_time.tick(duration);
                accelerator.get_velocity(&mut self.velocity, &duration.as_secs_f32());
            }
            if accelerator.burn_time.finished() {
                self.accelerator = None;
            }
        }
    }
}

pub fn straight_upward_moving_projectile(image: Handle<Image>) -> Projectile {
    let mut velocity = Velocity(Vec2::from((
        -25. + rand::random::<f32>() * 50.,
        250. + rand::random::<f32>() * 50.,
    )));
    let life = Lifetime(Timer::from_seconds(3. + rand::random::<f32>() * 1., false));
    let thrust = 1000. + rand::random::<f32>() * 500.;
    let burn_time = Timer::from_seconds(1. + rand::random::<f32>() * 0.5, false);
    let accelerator = Accelerator {
        accelerator: negative_gravity,
        burn_time: burn_time,
        thrust: thrust,
    };
    accelerator.get_velocity(&mut velocity, &0.);
    Projectile {
        velocity,
        life,
        image,
        color: None,
        accelerator: Some(accelerator),
    }
}