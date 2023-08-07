use bevy_ecs::system::ResMut;
use bevy_tokio_tasks::TokioTasksRuntime;
use crate::api_server;

pub fn setup_axum(runtime: ResMut<TokioTasksRuntime>) {
   runtime.spawn_background_task(|_ctx| async move {
        api_server().await;
    });


}