use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    boid::Boid,
    movement::{Acceleration, Velocity},
    neighborhood::{Neighborhood, Neighboring},
    schedule::InGameSet,
    steering_behaviors::{Alignment, Cohesion, Separation, SteeringBehavior},
};

const FLOCK_SIZE: usize = 100;
const START_RADIUS: f32 = 200.;
const SCAN_ANGLE: f32 = PI * 5. / 6.;
const SCAN_DISTANCE: f32 = 60.0;

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
            Velocity(Vec2::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
            )),
            Acceleration(Vec2::ZERO),
            Boid,
            Neighborhood {
                angle: SCAN_ANGLE,
                distance: SCAN_DISTANCE,
            },
            Separation {
                effectiveness: 1.0,
                ..default()
            },
            Alignment {
                effectiveness: 1.0,
                ..default()
            },
            Cohesion {
                effectiveness: 1.0,
                ..default()
            },
            Name::new("Traveler"),
        ));
    }
}

fn update_separation(
    mut boids: Query<(&GlobalTransform, &Velocity, &Neighborhood, &mut Separation)>,
    neighbors: Query<&GlobalTransform, With<Boid>>,
) {
    for (boid_transform, boid_velocity, neighborhood, mut separation) in boids.iter_mut() {
        let boid_position: Vec2 = boid_transform.translation().xy();

        let mut flockmates_relative_location: Vec<Vec2> = Vec::new();
        for neighbor_transform in neighbors.iter() {
            let neighbor_position: Vec2 = neighbor_transform.translation().xy();

            if neighborhood.are_neighbors(&boid_position, &boid_velocity.0, &neighbor_position) {
                flockmates_relative_location.push(neighbor_position - boid_position);
            }
        }

        separation.set_affecting_vector(&flockmates_relative_location);
    }
}

fn update_alignment(
    mut boids: Query<(&GlobalTransform, &Velocity, &Neighborhood, &mut Alignment)>,
    neighbors: Query<(&GlobalTransform, &Velocity), With<Boid>>,
) {
    for (boid_transform, boid_velocity, neighborhood, mut aligment) in boids.iter_mut() {
        let boid_position: Vec2 = boid_transform.translation().xy();

        let mut flockmates_relative_heading: Vec<Vec2> = Vec::new();
        for (neighbor_transform, neighbor_velocity) in neighbors.iter() {
            let neighbor_position: Vec2 = neighbor_transform.translation().xy();

            if neighborhood.are_neighbors(&boid_position, &boid_velocity.0, &neighbor_position) {
                flockmates_relative_heading.push(neighbor_velocity.0);
            }
        }

        aligment.set_affecting_vector(&flockmates_relative_heading);
    }
}

fn update_cohesion(
    mut boids: Query<(&GlobalTransform, &Velocity, &Neighborhood, &mut Cohesion)>,
    neighbors: Query<&GlobalTransform, With<Boid>>,
) {
    for (boid_transform, boid_velocity, neighborhood, mut cohesion) in boids.iter_mut() {
        let boid_position: Vec2 = boid_transform.translation().xy();

        let mut flockmates_relative_location: Vec<Vec2> = Vec::new();
        for neighbor_transform in neighbors.iter() {
            let neighbor_position: Vec2 = neighbor_transform.translation().xy();

            if neighborhood.are_neighbors(&boid_position, &boid_velocity.0, &neighbor_position) {
                flockmates_relative_location.push(neighbor_position - boid_position);
            }
        }

        cohesion.set_affecting_vector(&flockmates_relative_location);
    }
}
