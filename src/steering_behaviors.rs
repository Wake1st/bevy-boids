use bevy::{prelude::*, render::render_resource::encase::private::Length};

#[derive(Component, Default, Debug)]
pub struct Separation {
    pub affecting_vector: Vec2,
    pub effectiveness: f32,
}

#[derive(Component, Default, Debug)]
pub struct Alignment {
    pub affecting_vector: Vec2,
    pub effectiveness: f32,
}

#[derive(Component, Default, Debug)]
pub struct Cohesion {
    pub affecting_vector: Vec2,
    pub effectiveness: f32,
}

pub trait SteeringBehavior {
    fn set_affecting_vector(&mut self, flockmates: &Vec<Vec2>) -> ();
}

impl SteeringBehavior for Separation {
    /// Flockmates are relative location
    fn set_affecting_vector(&mut self, flockmates: &Vec<Vec2>) -> () {
        let flock_size: f32 = flockmates.length() as f32;
        if flock_size == 0.0 {
            self.affecting_vector = Vec2::ZERO;
        } else {
            let location_sum: Vec2 = flockmates.iter().sum();
            let average_location: Vec2 = location_sum / flock_size;
    
            info!("avg: {:?}\tlen: {:?}", average_location, average_location.length());
            self.affecting_vector = average_location / average_location.length() * self.effectiveness;
        }
    }
}

impl SteeringBehavior for Alignment {
    /// Flockmates are heading and should be unit vectors
    fn set_affecting_vector(&mut self, flockmates: &Vec<Vec2>) -> () {
        let flock_size: f32 = flockmates.length() as f32;
        if flock_size == 0.0 {
            self.affecting_vector = Vec2::ZERO;
        } else {
            let heading_sum: Vec2 = flockmates.iter().sum();
            let average_heading: Vec2 = heading_sum / flock_size;

            info!("avg: {:?}", average_heading);
            self.affecting_vector = average_heading * self.effectiveness;
        }
    }
}

impl SteeringBehavior for Cohesion {
    /// Flockmates are relative location
    fn set_affecting_vector(&mut self, flockmates: &Vec<Vec2>) -> () {
        let flock_size: f32 = flockmates.length() as f32;
        if flock_size == 0.0 {
            self.affecting_vector = Vec2::ZERO;
        } else {
            let location_sum: Vec2 = flockmates.iter().sum();
            let average_location: Vec2 = location_sum / flock_size;

            info!("avg: {:?}", average_location);
            self.affecting_vector = average_location * self.effectiveness;
        }
    }
}
