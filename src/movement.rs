use bevy::prelude::*;

use crate::{
    boid::{Alignment, Boid, Cohesion, Separation},
    schedule::InGameSet,
};

const SPEED: f32 = 40.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_movement.in_set(InGameSet::UpdateMovement));
    }
}

fn update_movement(
    mut flock: Query<(Entity, &mut Transform), With<Boid>>,
    behaviors: Query<(&Separation, &Alignment, &Cohesion)>,
    time: Res<Time>,
) {
    for (entity, mut transform) in flock.iter_mut() {
        if let Ok((separation, alignment, cohesion)) = behaviors.get(entity) {
            let direction = transform.rotation * Vec3::Y;
            transform.translation += direction * SPEED * time.delta_secs();
            transform.rotate_local_z(
                (separation.delta_angle + alignment.delta_angle + cohesion.delta_angle)
                    * time.delta_secs(),
            );
        }
    }
}
