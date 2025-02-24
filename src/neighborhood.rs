use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct SeparationNeighborhood {
    pub distance: f32,
    pub angle: f32,
}

#[derive(Component, Default, Debug)]
pub struct AlignmentNeighborhood {
    pub distance: f32,
    pub angle: f32,
}

#[derive(Component, Default, Debug)]
pub struct CohesionNeighborhood {
    pub distance: f32,
    pub angle: f32,
}

pub trait Neighbors {
    fn are_neighbors(&self, a_position: &Vec2, a_heading: &Vec2, b_position: &Vec2) -> bool;
}

impl Neighbors for SeparationNeighborhood {
    fn are_neighbors(
        &self,
        a_position: &Vec2,
        a_heading: &Vec2,
        b_position: &Vec2,
    ) -> bool {
        let distance = a_position.distance(*b_position);
        let angle = a_heading.angle_to(*b_position - a_position).abs();
        
        distance < self.distance && angle < self.angle
    }
}

impl Neighbors for AlignmentNeighborhood {
    fn are_neighbors(
        &self,
        a_position: &Vec2,
        a_heading: &Vec2,
        b_position: &Vec2,
    ) -> bool {
        let distance = a_position.distance(*b_position);
        let angle = a_heading.angle_to(*b_position - a_position).abs();

        distance < self.distance && angle < self.angle
    }
}

impl Neighbors for CohesionNeighborhood {
    fn are_neighbors(
        &self,
        a_position: &Vec2,
        a_heading: &Vec2,
        b_position: &Vec2,
    ) -> bool {
        let distance = a_position.distance(*b_position);
        let angle = a_heading.angle_to(*b_position - a_position).abs();
        
        distance < self.distance && angle < self.angle
    }
}