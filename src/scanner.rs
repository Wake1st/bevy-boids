use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Scanner {
    pub distance: f32,
    pub angle: f32,
}

pub trait Scanning {
    fn has_flockmate(&self, scanner: &Scanner, neighbor: &GlobalTransform) -> bool;
}

impl Scanning for GlobalTransform {
    fn has_flockmate(&self, scanner: &Scanner, neighbor: &GlobalTransform) -> bool {
        let distance = self.translation().distance(neighbor.translation());
        let angle = self.translation().angle_between(neighbor.translation());

        distance < scanner.distance && angle < scanner.angle
    }
}
