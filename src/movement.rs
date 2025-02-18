use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    boid::Boid,
    schedule::InGameSet,
    steering_behaviors::{Alignment, Cohesion, Separation},
};

const SPEED: f32 = 1.;
const ACCELERATION_EFFECT: f32 = 1.0;
const MAX_ACCELERATION: f32 = 15.0;
const MAX_VELOCITY: f32 = 65.0;
const WRAP_OFFSET: f32 = 16.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (set_acceleration, update_velocity, update_position, apply_screen_wrap)
                .in_set(InGameSet::UpdateMovement),
        );
    }
}

#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct Acceleration(pub Vec2);

fn update_position(
    mut flock: Query<(&Velocity, &mut Transform), With<Boid>>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in flock.iter_mut() {
        // move forward
        transform.translation += SPEED * velocity.0.extend(0.0) * time.delta_secs();
        // info!("pos: {:?}\tvel: {:?}", transform.translation.xy(), velocity.0);

        // get the quaternion to rotate from the forward direction to the velocity
        let rotate_to_velocity = Quat::from_rotation_arc(Vec3::Y, velocity.0.normalize().extend(0.));
        // rotate to velocity
        transform.rotation = rotate_to_velocity;
    }
}

fn update_velocity(mut flock: Query<(&Acceleration, &mut Velocity), With<Boid>>, time: Res<Time>) {
    for (acceleration, mut velocity) in flock.iter_mut() {
        let vel: Vec2 = velocity.0 + ACCELERATION_EFFECT * acceleration.0 * time.delta_secs();
        velocity.0 = vel.normalize() * vel.length().min(MAX_VELOCITY);
        // info!("vel: {:?}\tacc: {:?}", velocity.0, acceleration.0);
    }
}

fn set_acceleration(
    mut flock: Query<(Entity, &mut Acceleration), With<Boid>>,
    behaviors: Query<(&Separation, &Alignment, &Cohesion)>,
) {
    for (entity, mut acceleration) in flock.iter_mut() {
        if let Ok((separation, alignment, cohesion)) = behaviors.get(entity) {
            // info!(
            //     "sep: {:?}\tali: {:?}\tcoh: {:?}", 
            //     separation.affecting_vector, 
            //     alignment.affecting_vector, 
            //     cohesion.affecting_vector
            // );
            let acc: Vec2 = separation.affecting_vector + alignment.affecting_vector + cohesion.affecting_vector;
            acceleration.0 = acc.normalize() * acc.length().min(MAX_ACCELERATION);
        }
    }
}

fn apply_screen_wrap(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut wrap_query: Query<&mut Transform>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let size = window.size() + WRAP_OFFSET;
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let position = transform.translation.xy();
        let wrapped = (position + half_size).rem_euclid(size) - half_size;
        transform.translation = wrapped.extend(transform.translation.z);
    }
}
