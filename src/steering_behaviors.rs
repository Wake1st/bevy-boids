use bevy::prelude::*;

use crate::movement::{Velocity, MAX_ACCELERATION, MAX_SPEED};

#[derive(Component, Default, Debug)]
pub struct Separation {
    pub steering_vector: Vec2,
    pub effectiveness: f32,
}

#[derive(Component, Default, Debug)]
pub struct Alignment {
    pub steering_vector: Vec2,
    pub effectiveness: f32,
}

#[derive(Component, Default, Debug)]
pub struct Cohesion {
    pub steering_vector: Vec2,
    pub effectiveness: f32,
}

pub trait SteeringBehavior {
    fn set_steering_vector(&mut self, flockmate_size: usize, flockmates: Vec2, boid_velocity: &Velocity, boid_position: Vec2) -> ();
}

impl SteeringBehavior for Separation {
    /// Flockmates are relative location
    fn set_steering_vector(&mut self, flockmate_size: usize, flockmates_vector: Vec2, boid_velocity: &Velocity, _boid_position: Vec2) -> () {
        if flockmate_size == 0 {
            self.steering_vector = Vec2::ZERO;
        } else {
            let flockmates_avg_position = flockmates_vector / flockmate_size as f32;
            let limited_position = flockmates_avg_position.normalize() * MAX_SPEED;
            let relative_velocity = limited_position - boid_velocity.0;
            self.steering_vector =
                relative_velocity.clamp_length_max(MAX_ACCELERATION) * self.effectiveness;
        }
    }
}

impl SteeringBehavior for Alignment {
    /// Flockmates are heading and should be unit vectors
    fn set_steering_vector(&mut self, flockmate_size: usize, flockmates_vector: Vec2, boid_velocity: &Velocity, _boid_position: Vec2) -> () {
        if flockmate_size == 0 {
            self.steering_vector = Vec2::ZERO;
        } else {
            let flockmates_avg_velocity = flockmates_vector / flockmate_size as f32;
            let limited_flockmates_velocity = flockmates_avg_velocity.normalize() * MAX_SPEED;
            let relative_velocity = limited_flockmates_velocity - boid_velocity.0;
            self.steering_vector =
                relative_velocity.clamp_length_max(MAX_ACCELERATION) * self.effectiveness;
        }
    }
}

impl SteeringBehavior for Cohesion {
    /// Flockmates are relative location
    fn set_steering_vector(&mut self, flockmate_size: usize, flockmates_vector: Vec2, boid_velocity: &Velocity, boid_position: Vec2) -> () {
        if flockmate_size == 0 {
            self.steering_vector = Vec2::ZERO;
        } else {
            let flockmates_avg_position = flockmates_vector / flockmate_size as f32;
            let relative_position = flockmates_avg_position - boid_position;
            let limited_position = relative_position.normalize() * MAX_SPEED;
            let relative_velocity = limited_position - boid_velocity.0;
            self.steering_vector =
                relative_velocity.clamp_length_max(MAX_ACCELERATION) * self.effectiveness;
        }
    }
}
