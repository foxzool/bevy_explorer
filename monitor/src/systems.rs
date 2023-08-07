use crate::EcsMonitorPlugin;
use bevy::diagnostic::{DiagnosticMeasurement, DiagnosticsStore};
use bevy::utils::Instant;
use bevy_ecs::prelude::World;

pub fn world_entities_count(world: &mut World) {
    let ecs_count = world.entities().len();
    let mut diagnostics = world.resource_mut::<DiagnosticsStore>();
    if let Some(diagnostic) = diagnostics.get_mut(EcsMonitorPlugin::WORLD_ENTITIES_COUNT) {
        let measurement = DiagnosticMeasurement {
            time: Instant::now(),
            value: ecs_count as f64,
        };
        diagnostic.add_measurement(measurement)
    }
}
