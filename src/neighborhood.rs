use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Neighborhood {
    pub distance: f32,
    pub angle: f32,
}

pub trait Neighboring {
    fn are_neighbors(&self, a_position: &Vec2, a_heading: &Vec2, b_position: &Vec2) -> bool;
}

impl Neighboring for Neighborhood {
    fn are_neighbors(&self, a_position: &Vec2, a_heading: &Vec2, b_position: &Vec2) -> bool {
        let distance = a_position.distance(*b_position);
        let angle = a_heading.angle_to(*b_position);

        distance < self.distance && angle < self.angle
    }
}
