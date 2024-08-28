use bevy::{
    prelude::*,
    window::{AppLifecycle, WindowMode},
};

// the `bevy_main` proc_macro generates the required boilerplate for iOS and Android
#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            // on iOS, gestures must be enabled.
            // This doesn't work on Android
            recognize_rotation_gesture: true,
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, (setup_scene))
    .add_systems(Update, (handle_lifetime));

    // MSAA makes some Android devices panic, this is under investigation
    // https://github.com/bevyengine/bevy/issues/8229
    #[cfg(target_os = "android")]
    app.insert_resource(Msaa::Off);

    app.run();
}

fn setup_scene(mut commands: Commands) {
}

fn handle_lifetime(mut lifecycle_events: EventReader<AppLifecycle>) {
    for event in lifecycle_events.read() {
        match event {
            AppLifecycle::Idle | AppLifecycle::WillSuspend | AppLifecycle::WillResume => {}
            AppLifecycle::Suspended => {}
            AppLifecycle::Running => {}
        }
    }
}
