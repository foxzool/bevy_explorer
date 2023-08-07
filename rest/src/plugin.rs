use bevy::app::{App, Plugin, Startup};
use bevy_tokio_tasks::TokioTasksPlugin;

use bevy_ecs_monitor::EcsMonitorPlugin;

use crate::systems::setup_axum;

pub struct EcsRestPlugin;

impl Plugin for EcsRestPlugin {
    fn build(&self, app: &mut App) {
        // let tr = tokio::runtime::Runtime::new().unwrap();
        // tr.spawn(async {
        //     crate::api_server().await;
        // });

        // tr.join().unwrap();

        if !app.is_plugin_added::<TokioTasksPlugin>() {
            app.add_plugins(TokioTasksPlugin::default());
        }

        app.add_plugins(EcsMonitorPlugin).
            add_systems(Startup, setup_axum);
    }
}