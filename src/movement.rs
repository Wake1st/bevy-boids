use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    boid::Boid,
    schedule::InGameSet,
    steering_behaviors::{Alignment, Cohesion, Separation},
};

pub const MAX_ACCELERATION: f32 = 0.08;
pub const MAX_SPEED: f32 = 12.0;
const WRAP_OFFSET: f32 = 0.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_position,
                update_velocity,
                set_acceleration,
                apply_screen_wrap,
            )
                .in_set(InGameSet::UpdateMovement),
        );
    }
}

#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct Acceleration(pub Vec2);

fn update_position(mut flock: Query<(&Velocity, &mut Transform), With<Boid>>) {
    for (velocity, mut transform) in flock.iter_mut() {
        // move forward
        transform.translation += velocity.0.extend(0.0);

        // get the quaternion to rotate from the forward direction to the velocity
        let rotate_to_velocity =
            Quat::from_rotation_arc(Vec3::Y, velocity.0.normalize().extend(0.));
        // rotate to velocity
        transform.rotation = rotate_to_velocity;
    }
}

fn update_velocity(mut flock: Query<(&Acceleration, &mut Velocity), With<Boid>>) {
    for (acceleration, mut velocity) in flock.iter_mut() {
        velocity.0 += acceleration.0;
        velocity.0 = velocity.0.clamp_length_max(MAX_SPEED);
    }
}

fn set_acceleration(
    mut flock: Query<(Entity, &mut Acceleration), With<Boid>>,
    behaviors: Query<(&Separation, &Alignment, &Cohesion)>,
) {
    for (entity, mut acceleration) in flock.iter_mut() {
        if let Ok((separation, alignment, cohesion)) = behaviors.get(entity) {
            acceleration.0 =
                separation.steering_vector + alignment.steering_vector + cohesion.steering_vector;
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
    let half_width = size.x / 2.0;
    let half_height = size.y / 2.0;

    for mut transform in &mut wrap_query {
        let mut position = transform.translation.xy();

        if position.x < -half_width {
            position.x = half_width;
        } else if position.x > half_width {
            position.x = -half_width;
        }
        if position.y < -half_height {
            position.y = half_height;
        } else if position.y > half_height {
            position.y = -half_height;
        }

        transform.translation = position.extend(0.0);
    }
}
