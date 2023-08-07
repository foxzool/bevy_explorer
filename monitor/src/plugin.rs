use crate::systems::world_entities_count;
use bevy::app::{App, Plugin, Update};
use bevy::diagnostic::{Diagnostic, DiagnosticId, RegisterDiagnostic};

pub struct EcsMonitorPlugin;

impl Plugin for EcsMonitorPlugin {
    fn build(&self, app: &mut App) {
        app.register_diagnostic(Diagnostic::new(
            Self::WORLD_ENTITIES_COUNT,
            "world_ecs_count",
            10,
        ))
        .add_systems(Update, world_entities_count);
    }
}

impl EcsMonitorPlugin {
    pub const WORLD_ENTITIES_COUNT: DiagnosticId =
        DiagnosticId::from_u128(117259298154673752838376838005950978623);
}
