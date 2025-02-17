pub mod boid;
mod flock;
mod movement;
mod scanner;
pub mod schedule;

use bevy::prelude::*;
use flock::FlockPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;

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
    commands.spawn(Camera2d);
}
