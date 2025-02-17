use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    boid::{Alignment, Boid, BoidBehavior, Cohesion, Separation},
    scanner::{Scanner, Scanning},
    schedule::InGameSet,
};

const FLOCK_SIZE: usize = 100;
const START_RADIUS: f32 = 100.;
const SCAN_ANGLE: f32 = PI * 5. / 6.;
const SCAN_DISTANCE: f32 = 20.0;

pub struct FlockPlugin;

impl Plugin for FlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_flock).add_systems(
            Update,
            (update_separation, update_alignment, update_cohesion)
                .in_set(InGameSet::UpdateBehaviors),
        );
    }
}

fn spawn_flock(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("try angle ship.png");
    let mut rng = rand::rng();

    for _ in 0..FLOCK_SIZE {
        commands.spawn((
            Sprite {
                image: texture.clone(),
                ..default()
            },
            Transform::from_xyz(
                rng.random_range(-START_RADIUS..START_RADIUS),
                rng.random_range(-START_RADIUS..START_RADIUS),
                0.0,
            ),
            Boid,
            Scanner {
                angle: SCAN_ANGLE,
                distance: SCAN_DISTANCE,
            },
            Separation {
                effectiveness: 0.2,
                ..default()
            },
            Alignment {
                effectiveness: 0.2,
                ..default()
            },
            Cohesion {
                effectiveness: 0.1,
                ..default()
            },
            Name::new("Traveler"),
        ));
    }
}

fn update_separation(
    mut boids: Query<(&GlobalTransform, &Scanner, &mut Separation)>,
    neighbors: Query<&GlobalTransform, With<Boid>>,
) {
    for (boid_transform, scanner, mut separation) in boids.iter_mut() {
        let mut flockmates_relative_location: Vec<Vec2> = Vec::new();
        for neighbor_transform in neighbors.iter() {
            if boid_transform.has_flockmate(scanner, neighbor_transform) {
                flockmates_relative_location.push(
                    neighbor_transform.translation().xy() - boid_transform.translation().xy(),
                );
            }
        }

        separation.update_angle(
            boid_transform.translation().xy(),
            &flockmates_relative_location,
        );
    }
}

fn update_alignment(
    mut boids: Query<(&GlobalTransform, &Scanner, &mut Alignment)>,
    neighbors: Query<&GlobalTransform, With<Boid>>,
) {
    for (boid_transform, scanner, mut aligment) in boids.iter_mut() {
        let mut flockmates_relative_heading: Vec<Vec2> = Vec::new();
        for neighbor_transform in neighbors.iter() {
            if boid_transform.has_flockmate(scanner, neighbor_transform) {
                flockmates_relative_heading.push((neighbor_transform.rotation() * Vec3::Y).xy());
            }
        }

        aligment.update_angle(
            boid_transform.translation().xy(),
            &flockmates_relative_heading,
        );
    }
}

fn update_cohesion(
    mut boids: Query<(&GlobalTransform, &Scanner, &mut Cohesion)>,
    neighbors: Query<&GlobalTransform, With<Boid>>,
) {
    for (boid_transform, scanner, mut cohesion) in boids.iter_mut() {
        let mut flockmates_relative_location: Vec<Vec2> = Vec::new();
        for neighbor_transform in neighbors.iter() {
            if boid_transform.has_flockmate(scanner, neighbor_transform) {
                flockmates_relative_location.push(
                    neighbor_transform.translation().xy() - boid_transform.translation().xy(),
                );
            }
        }

        cohesion.update_angle(
            boid_transform.translation().xy(),
            &flockmates_relative_location,
        );
    }
}
