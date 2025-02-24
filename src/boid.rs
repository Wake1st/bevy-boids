use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{neighborhood::{AlignmentNeighborhood, CohesionNeighborhood, SeparationNeighborhood}, steering_behaviors::{Alignment, Cohesion, Separation}};

const SCAN_ANGLE: f32 = PI * 2. / 3.;
const SEPARATION_EFFECTIVENESS: f32 = 1.8;
const ALIGNMENT_EFFECTIVENESS: f32 = 0.9;
const COHESION_EFFECTIVENESS: f32 = 0.6;

const SEPARATION_DISTANCE: f32 = 40.;
const ALIGNMENT_DISTANCE: f32 = 60.;
const COHESION_DISTANCE: f32 = 80.;

#[derive(Component, Default, Debug)]
pub struct Boid;

#[derive(Bundle)]
pub struct BoidBundle {
    pub boid: Boid,
    pub separation_neighborhood: SeparationNeighborhood,
    pub separation: Separation,
    pub alignment_neighborhood: AlignmentNeighborhood,
    pub alignment: Alignment,
    pub cohesion_neighborhood: CohesionNeighborhood,
    pub cohesion: Cohesion,
    pub name: Name,
}

impl Default for BoidBundle {
    fn default() -> Self {
        Self { 
            boid: Boid,
            separation_neighborhood: SeparationNeighborhood {
                angle: SCAN_ANGLE,
                distance: SEPARATION_DISTANCE,
            },
            separation: Separation {
                effectiveness: SEPARATION_EFFECTIVENESS,
                ..default()
            },
            alignment_neighborhood: AlignmentNeighborhood {
                angle: SCAN_ANGLE,
                distance: ALIGNMENT_DISTANCE
            },
            alignment: Alignment {
                effectiveness: ALIGNMENT_EFFECTIVENESS,
                ..default()
            },
            cohesion_neighborhood: CohesionNeighborhood {
                angle: SCAN_ANGLE,
                distance: COHESION_DISTANCE,
            },
            cohesion: Cohesion {
                effectiveness: COHESION_EFFECTIVENESS,
                ..default()
            },
            name: Name::new("Traveler"),
        }
    }
}