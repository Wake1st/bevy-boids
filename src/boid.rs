use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Boid;

pub trait BoidBehavior {
    fn update_angle(&mut self, boid: Vec2, flockmates: &Vec<Vec2>) -> ();
}

#[derive(Component, Default, Debug)]
pub struct Separation {
    pub delta_angle: f32,
    pub effectiveness: f32,
}

impl BoidBehavior for Separation {
    /// Flockmates are relative location
    fn update_angle(&mut self, boid: Vec2, flockmates: &Vec<Vec2>) -> () {
        let flock_size: f32 = flockmates.len() as f32;
        let location_sum: Vec2 = flockmates.iter().sum();
        let average_location: Vec2 = location_sum / flock_size;

        self.delta_angle =
            boid.angle_to(average_location) * self.effectiveness / average_location.length();
    }
}

#[derive(Component, Default, Debug)]
pub struct Alignment {
    pub delta_angle: f32,
    pub effectiveness: f32,
}

impl BoidBehavior for Alignment {
    /// Flockmates are heading and should be unit vectors
    fn update_angle(&mut self, boid: Vec2, flockmates: &Vec<Vec2>) -> () {
        let flock_size: f32 = flockmates.len() as f32;
        let heading_sum: Vec2 = flockmates.iter().sum();
        let average_heading: Vec2 = heading_sum / flock_size;

        self.delta_angle = boid.angle_to(average_heading) * self.effectiveness;
    }
}

#[derive(Component, Default, Debug)]
pub struct Cohesion {
    pub delta_angle: f32,
    pub effectiveness: f32,
}

impl BoidBehavior for Cohesion {
    /// Flockmates are relative location
    fn update_angle(&mut self, boid: Vec2, flockmates: &Vec<Vec2>) -> () {
        let flock_size: f32 = flockmates.len() as f32;
        let location_sum: Vec2 = flockmates.iter().sum();
        let average_location: Vec2 = location_sum / flock_size;

        self.delta_angle = boid.angle_to(average_location) * self.effectiveness;
    }
}
