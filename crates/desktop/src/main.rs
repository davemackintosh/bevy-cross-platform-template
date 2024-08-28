use bevy::{prelude::*, window::WindowMode};
use core::{
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            mode: WindowMode::Windowed,
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, setup_scene);

    app.run();
}

fn setup_scene(
    mut commands: Commands,
    mut app_state: ResMut<AppState>,
    asset_server: Res<AssetServer>,
) {
}
