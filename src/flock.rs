use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    boid::Boid,
    movement::{Acceleration, Velocity},
    neighborhood::{AlignmentNeighborhood, CohesionNeighborhood, Neighbors, SeparationNeighborhood},
    schedule::InGameSet,
    steering_behaviors::{Alignment, Cohesion, Separation, SteeringBehavior},
};

const FLOCK_SIZE: usize = 240;
const START_RADIUS: f32 = 400.;
const START_VELOCITY: f32 = 4.;

const SCAN_ANGLE: f32 = PI * 2. / 3.;
const SEPARATION_EFFECTIVENESS: f32 = 1.8;
const ALIGNMENT_EFFECTIVENESS: f32 = 1.0;
const COHESION_EFFECTIVENESS: f32 = 0.4;

const SEPARATION_DISTANCE: f32 = 40.;
const ALIGNMENT_DISTANCE: f32 = 60.;
const COHESION_DISTANCE: f32 = 80.;

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
            Velocity(
                Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0))
                    .clamp_length_min(-START_VELOCITY),
            ),
            Acceleration(Vec2::ZERO),
            Boid,
            SeparationNeighborhood {
                angle: SCAN_ANGLE,
                distance: SEPARATION_DISTANCE,
            },
            Separation {
                effectiveness: SEPARATION_EFFECTIVENESS,
                ..default()
            },
            AlignmentNeighborhood {
                angle: SCAN_ANGLE,
                distance: ALIGNMENT_DISTANCE
            },
            Alignment {
                effectiveness: ALIGNMENT_EFFECTIVENESS,
                ..default()
            },
            CohesionNeighborhood {
                angle: SCAN_ANGLE,
                distance: COHESION_DISTANCE,
            },
            Cohesion {
                effectiveness: COHESION_EFFECTIVENESS,
                ..default()
            },
            Name::new("Traveler"),
        ));
    }
}

fn update_separation(
    mut boids: Query<(Entity, &GlobalTransform, &Velocity, &mut Separation, &SeparationNeighborhood)>,
    neighbors: Query<(Entity, &GlobalTransform), With<Boid>>,
) {
    for (boid_entity, boid_transform, boid_velocity, mut separation, neighborhood) in boids.iter_mut() {
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
                &boid_velocity.0,
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
        separation.set_steering_vector(flockmate_size, flockmates_combined_location, boid_velocity, boid_position);
    }
}

fn update_alignment(
    mut boids: Query<(Entity, &GlobalTransform, &Velocity, &mut Alignment, &AlignmentNeighborhood)>,
    neighbors: Query<(Entity, &GlobalTransform, &Velocity), With<Boid>>,
) {
    for (boid_entity, boid_transform, boid_velocity, mut alignment, neighborhood) in boids.iter_mut() {
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
                &boid_velocity.0,
                &neighbor_position,
            ) {
                flockmates_combined_heading += neighbor_velocity.0;
                flockmate_size += 1;
            }
        }

        //  if no flockmates are present, then there's no vector to steer with
        alignment.set_steering_vector(flockmate_size, flockmates_combined_heading, boid_velocity, boid_position);
    }
}

fn update_cohesion(
    mut boids: Query<(Entity, &GlobalTransform, &Velocity, &mut Cohesion, &CohesionNeighborhood)>,
    neighbors: Query<(Entity, &GlobalTransform), With<Boid>>,
) {
    for (boid_entity, boid_transform, boid_velocity, mut cohesion, neighborhood) in boids.iter_mut() {
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
                &boid_velocity.0,
                &neighbor_position,
            ) {
                flockmates_combined_location += neighbor_position;
                flockmate_size += 1;
            }
        }

        //  if no flockmates are present, then there's no vector to steer with
        cohesion.set_steering_vector(flockmate_size, flockmates_combined_location, boid_velocity, boid_position);
    }
}
