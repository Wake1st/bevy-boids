use bevy::prelude::*;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::Input,
                InGameSet::UpdateBehaviors,
                InGameSet::UpdateMovement,
            )
                .chain(),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    Input,
    UpdateBehaviors,
    UpdateMovement,
}
