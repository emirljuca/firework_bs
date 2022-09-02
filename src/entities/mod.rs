use std::time::Duration;

use bevy::{prelude::*, render::render_graph::GraphInputNode};

const GRAVITY: Vec2 = Vec2::new(0., -60.);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone)]
pub struct Rotation(pub f32);

#[derive(Component, Clone)]
pub struct Lifetime(pub Timer);

// Accelerator: a(t) = dv/dt -> dv = dt*a(t)
// Velocity: v = a*t + cv ; v = dp/dt ; v(dt+t) = v(t) + v(dt)*dt
// Position: p = a*t^2 + cv*t + cp

fn randomize_accelerator(
    thrust: &f32,
    velocity: &mut Velocity,
    d_time: &f32,
    burn_time: &Timer,
) {
    let acceleration = *thrust * velocity.0.normalize() * rand::random::<f32>();
    velocity.0 += acceleration * *d_time;
}

fn exponential_accelerator(
    thrust: &f32,
    velocity: &mut Velocity,
    d_time: &f32,
    burn_time: &Timer,
) {
    let acceleration = *thrust * velocity.0.normalize() * (-(burn_time.elapsed().as_millis() as f32) / 500.).exp();
    // let acceleration = *thrust * velocity.0.normalize();
    velocity.0 += acceleration * *d_time;
}

#[derive(Component, Clone)]
pub struct Accelerator {
    pub accelerator:
        fn(thrust: &f32,
            velocity: &mut Velocity,
            d_time: &f32,
            burn_time: &Timer,),
    pub burn_time: Timer,
    pub thrust: f32
}

impl Accelerator {
    pub fn get_velocity(&self, velocity: &mut Velocity, d_time: &f32) {
        (self.accelerator)(&self.thrust, velocity, d_time, &self.burn_time);
    }
}

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
        self.velocity.0 += GRAVITY*duration.as_secs_f32();
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

fn straight_upward_moving_projectile(image: Handle<Image>) -> Projectile {
    let mut velocity = Velocity(Vec2::from((
        -5. + rand::random::<f32>() * 10.,
    50. + rand::random::<f32>() * 10.,
    )));
    let life = Lifetime(Timer::from_seconds(4. + rand::random::<f32>() * 1., false));
    let thrust = 1000. + rand::random::<f32>() * 500.;
    let burn_time = Timer::from_seconds(
        0.25 + rand::random::<f32>() * 0.1,
        false,
    );
    let accelerator = Accelerator {
        accelerator: exponential_accelerator,
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
) -> Vec<Shell> {
    let mut shells = Vec::new();
    for _ in 0..20 {
        let speed = 25. + rand::random::<f32>() * 25.;
        let life = Lifetime(Timer::from_seconds(2. + rand::random::<f32>() * 1., false));
        let angle = std::f32::consts::TAU * rand::random::<f32>();
        let mut velocity = Velocity(speed * Vec2::from((angle.cos(), angle.sin())));
        let thrust = 100. + rand::random::<f32>() * 50.;
        let burn_time = Timer::from_seconds(
            0.5 + rand::random::<f32>() * 0.5,
            false,
        );
        let accelerator = Accelerator {
            accelerator: randomize_accelerator,
            burn_time: burn_time,
            thrust: thrust,
        };
        accelerator.get_velocity( &mut velocity, &0.);
        let projectile = Projectile {
            velocity,
            life: life,
            image: image.clone(),
            color: None,
            accelerator: Some(accelerator),
        };
        shells.push(Shell {
            projectile,
            shells: None,
        });
    }
    shells
}

pub fn ClassicFirework(asset_server: &Res<AssetServer>) -> Shell {
    let projectile = straight_upward_moving_projectile(asset_server.load("bubble.png"));
    let shells = blooming_shells(asset_server.load("bubble.png"), None, None);
    Shell {
        projectile,
        shells: Some(Shells { shells }),
    }
}
