pub mod boid;
mod flock;
mod movement;
mod neighborhood;
pub mod schedule;
pub mod steering_behaviors;

use bevy::prelude::*;
use flock::FlockPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;

const CAMERA_DISTANCE: f32 = 1000.;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_systems(Startup, setup_camera)
            .add_plugins((SchedulePlugin, FlockPlugin, MovementPlugin));
    }
}

fn setup_camera(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            near: -CAMERA_DISTANCE,
            far: CAMERA_DISTANCE,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
