use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    boid::{Boid, BoidBundle},
    movement::{Acceleration, Velocity},
    neighborhood::{AlignmentNeighborhood, CohesionNeighborhood, Neighbors, SeparationNeighborhood},
    schedule::InGameSet,
    steering_behaviors::{Alignment, Cohesion, Separation, SteeringBehavior},
};

const FLOCK_SIZE: usize = 240;
const START_RADIUS: f32 = 400.;
const START_VELOCITY: f32 = 6.;
const MAX_SPEED: f32 = 12.0;
const MAX_ACCELERATION: f32 = 0.2;

const BIG_FLOCK_SIZE: usize = 10;
const BIG_START_VELOCITY: f32 = 2.;
const BIG_MAX_SPEED: f32 = 2.0;
const BIG_MAX_ACCELERATION: f32 = 0.02;

const BIG_SCAN_ANGLE: f32 = PI / 3.0;
const BIG_SEPARATION_EFFECTIVENESS: f32 = 0.2;
const BIG_ALIGNMENT_EFFECTIVENESS: f32 = 0.1;
const BIG_COHESION_EFFECTIVENESS: f32 = 0.1;

const BIG_SEPARATION_DISTANCE: f32 = 180.;
const BIG_ALIGNMENT_DISTANCE: f32 = 100.;
const BIG_COHESION_DISTANCE: f32 = 360.;

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
    let mut rng = rand::rng();
    
    let texture: Handle<Image> = asset_server.load("try angle ship.png");
    for _ in 0..FLOCK_SIZE {
        commands.spawn((
            Sprite::from_image(texture.clone()),
            Transform::from_xyz(
                rng.random_range(-START_RADIUS..START_RADIUS),
                rng.random_range(-START_RADIUS..START_RADIUS),
                0.0,
            ),
            Velocity {
                value: Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0))
                .clamp_length_min(-START_VELOCITY),
                max: MAX_SPEED
            },
            Acceleration {
                value: Vec2::ZERO,
                max: MAX_ACCELERATION,
            },
            BoidBundle::default()
        ));
    }

    let texture: Handle<Image> = asset_server.load("big ship.png");
    for _ in 0..BIG_FLOCK_SIZE {
        commands.spawn((
            Sprite::from_image(texture.clone()),
            Transform::from_xyz(
                rng.random_range(-START_RADIUS..START_RADIUS),
                rng.random_range(-START_RADIUS..START_RADIUS),
                0.0,
            ),
            Velocity {
                value: Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0))
                .clamp_length_min(-BIG_START_VELOCITY),
                max: BIG_MAX_SPEED
            },
            Acceleration {
                value: Vec2::ZERO,
                max: BIG_MAX_ACCELERATION,
            },
            BoidBundle {
                separation: Separation {
                    effectiveness: BIG_SEPARATION_EFFECTIVENESS,
                    ..default()
                },
                alignment: Alignment {
                    effectiveness: BIG_ALIGNMENT_EFFECTIVENESS,
                    ..default()
                },
                cohesion: Cohesion {
                    effectiveness: BIG_COHESION_EFFECTIVENESS,
                    ..default()
                },
                separation_neighborhood: SeparationNeighborhood {
                    distance: BIG_SEPARATION_DISTANCE,
                    angle: BIG_SCAN_ANGLE
                },
                alignment_neighborhood: AlignmentNeighborhood {
                    distance: BIG_ALIGNMENT_DISTANCE,
                    angle: BIG_SCAN_ANGLE
                },
                cohesion_neighborhood: CohesionNeighborhood {
                    distance: BIG_COHESION_DISTANCE,
                    angle: BIG_SCAN_ANGLE
                },
                ..default()
            }
        ));
    }
}

fn update_separation(
    mut boids: Query<(Entity, &GlobalTransform, &Velocity, &Acceleration, &mut Separation, &SeparationNeighborhood)>,
    neighbors: Query<(Entity, &GlobalTransform), With<Boid>>,
) {
    for (boid_entity, boid_transform, boid_velocity, boid_acceleration, mut separation, neighborhood) in boids.iter_mut() {
        let boid_position: Vec2 = boid_transform.translation().xy();
        let mut flockmates_combined_location: Vec2 = Vec2::ZERO;
        let mut flockmate_size: usize = 0;

        //  get average flockmate position
        for (neighbor_entity, neighbor_transform) in neighbors.iter() {
            //  do not allow boids to effect themselves
            if boid_entity == neighbor_entity {
                continue;
            }

            let neighbor_position: Vec2 = neighbor_transform.translation().xy();
            if neighborhood.are_neighbors(
                &boid_position,
                &boid_velocity.value,
                &neighbor_position,
            ) {
                let distance = boid_position.distance(neighbor_position);
                let relative_position = boid_position - neighbor_position;
                let inverse = relative_position / (distance * distance);
                flockmates_combined_location += inverse;
                flockmate_size += 1;
            }
        }

        //  if no flockmates are present, then there's no vector to steer with
        separation.set_steering_vector(flockmate_size, flockmates_combined_location, boid_velocity, boid_acceleration, boid_position);
    }
}

fn update_alignment(
    mut boids: Query<(Entity, &GlobalTransform, &Velocity, &Acceleration, &mut Alignment, &AlignmentNeighborhood)>,
    neighbors: Query<(Entity, &GlobalTransform, &Velocity), With<Boid>>,
) {
    for (boid_entity, boid_transform, boid_velocity, boid_acceleration, mut alignment, neighborhood) in boids.iter_mut() {
        let boid_position: Vec2 = boid_transform.translation().xy();
        let mut flockmates_combined_heading: Vec2 = Vec2::ZERO;
        let mut flockmate_size: usize = 0;

        //  get average flockmate heading vector
        for (neighbor_entity, neighbor_transform, neighbor_velocity) in neighbors.iter() {
            //  do not allow boids to effect themselves
            if boid_entity == neighbor_entity {
                continue;
            }

            let neighbor_position: Vec2 = neighbor_transform.translation().xy();
            if neighborhood.are_neighbors(
                &boid_position,
                &boid_velocity.value,
                &neighbor_position,
            ) {
                flockmates_combined_heading += neighbor_velocity.value;
                flockmate_size += 1;
            }
        }

        //  if no flockmates are present, then there's no vector to steer with
        alignment.set_steering_vector(flockmate_size, flockmates_combined_heading, boid_velocity, boid_acceleration, boid_position);
    }
}

fn update_cohesion(
    mut boids: Query<(Entity, &GlobalTransform, &Velocity, &Acceleration, &mut Cohesion, &CohesionNeighborhood)>,
    neighbors: Query<(Entity, &GlobalTransform), With<Boid>>,
) {
    for (boid_entity, boid_transform, boid_velocity, boid_acceleration, mut cohesion, neighborhood) in boids.iter_mut() {
        let boid_position: Vec2 = boid_transform.translation().xy();
        let mut flockmates_combined_location: Vec2 = Vec2::ZERO;
        let mut flockmate_size: usize = 0;

        //  get average flockmate position
        for (neighbor_entity, neighbor_transform) in neighbors.iter() {
            //  do not allow boids to effect themselves
            if boid_entity == neighbor_entity {
                continue;
            }

            let neighbor_position: Vec2 = neighbor_transform.translation().xy();
            if neighborhood.are_neighbors(
                &boid_position,
                &boid_velocity.value,
                &neighbor_position,
            ) {
                flockmates_combined_location += neighbor_position;
                flockmate_size += 1;
            }
        }

        //  if no flockmates are present, then there's no vector to steer with
        cohesion.set_steering_vector(flockmate_size, flockmates_combined_location, boid_velocity, boid_acceleration, boid_position);
    }
}
