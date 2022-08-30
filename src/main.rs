use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy::window::WindowMode;
mod entities;
use entities::*;

const HEIGHT: f32 = 1440.0;
const WIDTH: f32 = 2560.0;
const SPEED: f32 = 250.;
const ROTATION: f32 = 2.;
const LIFE: f32 = 25.;

fn main() {
    App::new()
        .insert_resource(bevy::render::texture::ImageSettings::default_nearest())
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            scale_factor_override: Some(1.),
            title: "Test Project : plz compile".to_string(),
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(handle_mouse_events)
        .add_system(movement)
        .add_system(life)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn handle_mouse_events(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().expect("no primary window");
        let position = window.cursor_position().expect("clicked out of window")
            - Vec2::from((WIDTH / 2., HEIGHT / 2.));
        let firework = ClassicFirework(&asset_server);
        // let sprite: Handle<Image> = asset_server.load("sprite.png");
        // let bubble: Handle<Image> = asset_server.load("bubble.png");
        // spawn_particles(&mut commands, sprite, 10, position, SPEED, ROTATION, 0.5*LIFE);
        spawn_shells(&mut commands, position, vec![firework]);
        // spawn_particles(&mut commands, bubble, 1, position, 0., ROTATION, 0.5*LIFE);
    }
}

fn spawn_shells(commands: &mut Commands, position: Vec2, shells: Vec<Shell>) {
    for shell in shells {
        let transform = Transform {
            translation: Vec3::from((position, 0.)),
            scale: Vec3::splat(2.),
            rotation: Quat::from_rotation_z(0.),
        };
        commands
            .spawn_bundle(SpriteBundle {
                texture: shell.projectile.image.clone(),
                transform,
                ..default()
            })
            .insert(shell.projectile.velocity)
            .insert(shell.projectile.life)
            .insert(shell.projectile.acceleration)
            .insert(shell.projectile.propulsion.unwrap_or_default())
            .insert(shell.shells.unwrap_or_default());
    }
}

fn movement(
    time: Res<Time>,
    mut query: Query<(
        &mut Acceleration,
        &mut Propulsion,
        &mut Velocity,
        &mut Transform,
    )>,
) {
    for (mut acceleration, mut propulsion, mut velocity, mut transform) in &mut query {
        // Continue initial acceleration of propulsed object until burn time is finished.
        if !propulsion.burn_time.finished() {
            // Tick forward burn time of propultion.
            propulsion.burn_time.tick(time.delta());
        }
        if propulsion.burn_time.finished() {
            if propulsion.thrust > 0. {
                // Deaccelerate projectile from thrust.
                acceleration.0 -= get_thrust_acceleration(&propulsion, &velocity);
                // Remove thrust.
                propulsion.thrust = 0.;
            }
        }
        // Calculate current velocity.
        velocity.0 += Vec2::from(acceleration.0 * time.delta_seconds());
        // Move object based off current
        transform.translation += Vec3::from((velocity.0 * time.delta_seconds(), 0.));
        if transform.translation.y > HEIGHT / 2. {
            velocity.0.y = -velocity.0.y.abs();
        } else if transform.translation.y < -HEIGHT / 2. {
            velocity.0.y = velocity.0.y.abs();
        }

        if transform.translation.x > WIDTH / 2. {
            velocity.0.x = -velocity.0.x.abs();
        } else if transform.translation.x < -WIDTH / 2. {
            velocity.0.x = velocity.0.x.abs();
        }
    }
}

fn life(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime, &Shells, &Transform)>,
) {
    for (entity, mut lifetime, shells, transform) in &mut query {
        lifetime.0.tick(time.delta());
        if lifetime.0.finished() {
            commands.entity(entity).despawn();
            let mut vec = shells.shells.clone();
            spawn_shells(&mut commands, Vec2::new(transform.translation.x, transform.translation.y), vec);
        }
    }
}
