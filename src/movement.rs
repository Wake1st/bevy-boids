use bevy::prelude::*;

use crate::{
    boid::Boid,
    schedule::InGameSet,
    steering_behaviors::{Alignment, Cohesion, Separation},
};

const SPEED: f32 = 1.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_position, update_velocity, update_acceleration)
                .in_set(InGameSet::UpdateMovement),
        );
    }
}

#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct Acceleration(pub Vec2);

fn update_position(
    mut flock: Query<(&Velocity, &GlobalTransform, &mut Transform), With<Boid>>,
    time: Res<Time>,
) {
    for (velocity, global_transform, mut transform) in flock.iter_mut() {
        // move forward
        transform.translation += SPEED * velocity.0.extend(0.0) * time.delta_secs();

        // face forward
        let target = velocity.0.extend(0.0) + global_transform.translation();
        let rotated_transform = transform.looking_at(target, Vec3::Z);
        *transform = rotated_transform;

        info!("translation: {:?}", transform.translation);
    }
}

fn update_velocity(mut flock: Query<(&Acceleration, &mut Velocity), With<Boid>>, time: Res<Time>) {
    for (acceleration, mut velocity) in flock.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_secs();
    }
}

fn update_acceleration(
    mut flock: Query<(Entity, &mut Acceleration), With<Boid>>,
    behaviors: Query<(&Separation, &Alignment, &Cohesion)>,
) {
    for (entity, mut acceleration) in flock.iter_mut() {
        if let Ok((separation, alignment, cohesion)) = behaviors.get(entity) {
            acceleration.0 = separation.affecting_vector
                + alignment.affecting_vector
                + cohesion.affecting_vector;
        }
    }
}
