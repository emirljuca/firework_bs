use bevy::prelude::*;

// Accelerator: a(t) = dv/dt -> dv = dt*a(t)
// Velocity: v = a*t + cv ; v = dp/dt ; v(dt+t) = v(t) + v(dt)*dt
// Position: p = a*t^2 + cv*t + cp

#[derive(Component, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone)]
pub struct Rotation(pub f32);

#[derive(Component, Clone)]
pub struct Lifetime(pub Timer);