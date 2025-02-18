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

const FLOCK_SIZE: usize = 40;
const START_RADIUS: f32 = 100.;
const START_VELOCITY: f32 = 60.0;

const SCAN_ANGLE: f32 = PI * 2./3.;
const SCAN_DISTANCE: f32 = 60.0;
const ALIGNMENT: f32 = 1.5;
const COHESION: f32 = 1.0;
const SEPARATION: f32 = 2.0;

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
            Sprite::from_image(texture.clone()),
            Transform::from_xyz(
                rng.random_range(-START_RADIUS..START_RADIUS),
                rng.random_range(-START_RADIUS..START_RADIUS),
                0.0,
            ),
            Velocity(Vec2::new(
                rng.random_range(-START_VELOCITY..START_VELOCITY),
                rng.random_range(-START_VELOCITY..START_VELOCITY),
            )),
            Acceleration(Vec2::ZERO),
            Boid,
            Neighborhood {
                angle: SCAN_ANGLE,
                distance: SCAN_DISTANCE,
            },
            Separation {
                effectiveness: SEPARATION,
                ..default()
            },
            Alignment {
                effectiveness: ALIGNMENT,
                ..default()
            },
            Cohesion {
                effectiveness: COHESION,
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

        // info!("flockmate count: {:?}", flockmates_relative_location.len());
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

            if neighborhood.are_neighbors(&boid_position, &boid_velocity.0.normalize(), &neighbor_position) {
                flockmates_relative_heading.push(neighbor_velocity.0);
            }
        }

        // info!("flockmate count: {:?}", flockmates_relative_heading.len());
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

        // info!("flockmate count: {:?}", flockmates_relative_location.len());
        cohesion.set_affecting_vector(&flockmates_relative_location);
    }
}
