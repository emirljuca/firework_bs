use bevy::prelude::*;

const GRAVITY: Vec2 = Vec2::new(0., -60.);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Clone)]
pub struct Rotation(pub f32);

#[derive(Component, Clone)]
pub struct Lifetime(pub Timer);

#[derive(Component, Clone)]
pub struct Propulsion {
    pub thrust: f32,
    pub burn_time: Timer,
}

impl Default for Propulsion {
    fn default() -> Self {
        Propulsion {
            thrust: 0.,
            burn_time: Timer::from_seconds(0., false),
        }
    }
}
#[derive(Clone)]
pub struct Projectile {
    pub velocity: Velocity,
    pub life: Lifetime,
    pub acceleration: Acceleration,
    pub image: Handle<Image>,
    pub color: Option<String>,
    pub propulsion: Option<Propulsion>,
}

pub fn get_thrust_acceleration(propulsion: &Propulsion, velocity: &Velocity) -> Vec2 {
    propulsion.thrust * velocity.0.normalize()
}

fn straight_upward_moving_projectile(image: Handle<Image>) -> Projectile {
    let velocity = Velocity(Vec2::from((-20. + rand::random::<f32>()*40., 250.+ rand::random::<f32>()*50.)));
    let life = Lifetime(Timer::from_seconds(2.+ rand::random::<f32>()*2., false));
    let propulsion = Propulsion {
        thrust: 100. + rand::random::<f32>()*50.,
        burn_time: Timer::from_seconds(0.5 + rand::random::<f32>()*0.5, false),
    };
    let prop_vec = get_thrust_acceleration(&propulsion, &velocity);
    let acceleration = Acceleration(prop_vec + GRAVITY);
    Projectile {
        velocity,
        life,
        acceleration,
        image,
        color: None,
        propulsion: Some(propulsion),
    }
}

#[derive(Component, Clone)]
pub struct Shells {
    pub shells: Vec<Shell>
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

fn blooming_shells(image: Handle<Image>, projectile_in: Option<Projectile>, number: Option<i32>) -> Vec<Shell> {
    let acceleration = Acceleration(GRAVITY);
    let mut shells = Vec::new();
    for _ in 0..20 {
        let speed = 50.+rand::random::<f32>()*50.;
        let life = Lifetime(Timer::from_seconds(1.5 + rand::random::<f32>()*2., false));
        let angle = std::f32::consts::TAU * rand::random::<f32>();
        let velocity = Velocity(speed * Vec2::from((angle.cos(), angle.sin())));
        let projectile = Projectile {
            velocity,
            life: life.clone(),
            acceleration: acceleration.clone(),
            image: image.clone(),
            color: None,
            propulsion: None,
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
        shells: Some(Shells{ shells }),
    }
}
