use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;

use bevy_ecs_explorer::EcsRestPlugin;

#[derive(Component)]
pub struct Ball {
    velocity: Vec3,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((LogDiagnosticsPlugin::default(), EcsRestPlugin))
        .add_systems(Update, mouse_handler)
        .run();
}

fn mouse_handler(mut commands: Commands, mouse_button_input: Res<Input<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        commands.spawn(Ball {
            velocity: Vec3::new(0.0, 0.0, 0.0),
        });
    }
}
