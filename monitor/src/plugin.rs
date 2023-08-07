use bevy::app::{App, Plugin, Update};
use bevy::diagnostic::{Diagnostic, RegisterDiagnostic};
use crate::systems::{world_entities_count, WORLD_ENTITIES_COUNT};

pub struct EcsMonitorPlugin;

impl Plugin for EcsMonitorPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_diagnostic(
                Diagnostic::new(WORLD_ENTITIES_COUNT, "world_ecs_count", 10)
                    .with_suffix(" count"),
            )
            .add_systems(Update, world_entities_count);
    }
}