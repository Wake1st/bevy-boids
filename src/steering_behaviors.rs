use bevy::{prelude::*, render::render_resource::encase::private::Length};

use crate::movement::MAX_ACCELERATION;

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
    fn set_steering_vector(&mut self, flockmates: &Vec<Vec2>) -> ();
}

impl SteeringBehavior for Separation {
    /// Flockmates are relative location
    fn set_steering_vector(&mut self, flockmates: &Vec<Vec2>) -> () {
        let flock_size: f32 = flockmates.length() as f32;
        if flockmate_size == 0 {
            separation.steering_vector = Vec2::ZERO;
        } else {
            let flockmates_avg_position = flockmates_combined_location / flockmate_size as f32;
            let limited_position = flockmates_avg_position.normalize() * MAX_SPEED;
            let relative_velocity = limited_position - boid_velocity.0;
            separation.steering_vector =
                relative_velocity.clamp_length_max(MAX_ACCELERATION) * self.effectiveness;
        }
    }
}

impl SteeringBehavior for Alignment {
    /// Flockmates are heading and should be unit vectors
    fn set_steering_vector(&mut self, flockmates: &Vec<Vec2>) -> () {
        let flock_size: f32 = flockmates.length() as f32;
        if flockmate_size == 0 {
            alignment.steering_vector = Vec2::ZERO;
        } else {
            let flockmates_avg_velocity = flockmates_combined_heading / flockmate_size as f32;
            let limited_flockmates_velocity = flockmates_avg_velocity.normalize() * MAX_SPEED;
            let relative_velocity = limited_flockmates_velocity - boid_velocity.0;
            alignment.steering_vector =
                relative_velocity.clamp_length_max(MAX_ACCELERATION) * self.effectiveness;
        }
    }
}

impl SteeringBehavior for Cohesion {
    /// Flockmates are relative location
    fn set_steering_vector(&mut self, flockmates: &Vec<Vec2>) -> () {
        let flock_size: f32 = flockmates.length() as f32;
        if flockmate_size == 0 {
            cohesion.steering_vector = Vec2::ZERO;
        } else {
            let flockmates_avg_position = flockmates_combined_location / flockmate_size as f32;
            let relative_position = flockmates_avg_position - boid_position;
            let limited_position = relative_position.normalize() * MAX_SPEED;
            let relative_velocity = limited_position - boid_velocity.0;
            cohesion.steering_vector =
                relative_velocity.clamp_length_max(MAX_ACCELERATION) * self.effectiveness;
        }
    }
}
