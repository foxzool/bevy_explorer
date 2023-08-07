use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;

use bevy_ecs_explorer::EcsRestPlugin;

fn main() {
    App::new().add_plugins(DefaultPlugins)
        .add_plugins((LogDiagnosticsPlugin::default() , EcsRestPlugin)).run();
}