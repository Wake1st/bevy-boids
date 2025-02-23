use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Neighborhood {
    pub distance: f32,
    pub angle: f32,
}

pub fn are_neighbors(
    a_position: &Vec2,
    a_heading: &Vec2,
    b_position: &Vec2,
    neighborhood: &Neighborhood,
) -> bool {
    let distance = a_position.distance(*b_position);
    let angle = a_position.angle_to(*b_position); //  why does this?
    // let angle = a_heading.angle_to(*b_position - a_position).abs();  // work better than this?

    // info!("a head: {:?}\tb pos: {:?}", a_heading, b_position);
    // info!("dist: {:?}\tangle: {:?}", distance, angle);
    distance < neighborhood.distance && angle < neighborhood.angle
}
