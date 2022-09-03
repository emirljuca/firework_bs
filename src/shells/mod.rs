use bevy::prelude::*;

use self::accelerator::*;
use self::projectile::*;
use self::movement::*;
pub mod movement;
pub mod accelerator;
pub mod projectile;

#[derive(Component, Clone)]
pub struct Shells {
    pub shells: Vec<Shell>,
}

impl Default for Shells {
    fn default() -> Shells {
        Shells { shells: Vec::new() }
    }
}

#[derive(Component, Clone)]
pub struct Shell {
    pub projectile: Projectile,
    pub shells: Option<Shells>,
}

fn blooming_shells(
    image: Handle<Image>,
    projectile_in: Option<Projectile>,
    number: Option<i32>,
    accelerator: &Accelerator,
) -> Vec<Shell> {
    let mut shells = Vec::new();
    let number_shells = number.unwrap_or(20);
    for _ in 0..number_shells {
        let speed = 25. + rand::random::<f32>() * 25.;
        let life = Lifetime(Timer::from_seconds(2. + rand::random::<f32>() * 1., false));
        let angle = std::f32::consts::TAU * rand::random::<f32>();
        let mut velocity = Velocity(speed * Vec2::from((angle.cos(), angle.sin())));
        accelerator.get_velocity(&mut velocity, &0.);
        let projectile = Projectile {
            velocity,
            life: life,
            image: image.clone(),
            color: None,
            accelerator: Some(accelerator.clone()),
        };
        shells.push(Shell {
            projectile,
            shells: None,
        });
    }
    shells
}

pub fn bees_firework(asset_server: &Res<AssetServer>) -> Shell {
    let projectile = straight_upward_moving_projectile(asset_server.load("firework.png"));
    let thrust = 8. + rand::random::<f32>() * 2.;
    let burn_time = Timer::from_seconds(1.5 + rand::random::<f32>() * 0.5, false);
    let accelerator = Accelerator {
        accelerator: randomize_accelerator,
        burn_time: burn_time,
        thrust: thrust,
    };
    let shells = blooming_shells(asset_server.load("firework_elements/sodium.png"), None, Some(50), &accelerator);
    Shell {
        projectile,
        shells: Some(Shells { shells }),
    }
}

pub fn classic_firework(asset_server: &Res<AssetServer>) -> Shell {
    let projectile = straight_upward_moving_projectile(asset_server.load("firework.png"));
    let thrust = 8. + rand::random::<f32>() * 2.;
    let burn_time = Timer::from_seconds(1.5 + rand::random::<f32>() * 0.5, false);
    let accelerator = Accelerator {
        accelerator: negative_gravity,
        burn_time: burn_time,
        thrust: thrust,
    };
    let shells = blooming_shells(asset_server.load("firework_elements/copper.png"), None, Some(40), &accelerator);
    Shell {
        projectile,
        shells: Some(Shells { shells }),
    }
}
