use avian3d::PhysicsPlugins;
use avian3d::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy::winit::WinitSettings;
use bevy_skein::SkeinPlugin;
use game_camera::CameraPlugin;
use game_player::PlayerPlugin;
use std::env::current_dir;

fn main() -> AppExit {
    let mut app = App::new();

    // Bevy plugins
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "RPG Automation".to_string(),
                    present_mode: PresentMode::AutoNoVsync,
                    // mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: format!("{}/assets", current_dir().unwrap().to_str().unwrap()),
                ..default()
            }),
    );
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(PlayerPlugin);
    app.add_plugins(CameraPlugin);

    // Avian
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(PhysicsDebugPlugin::default());
    app.insert_resource(Gravity(Vec3::new(0.0, -50.0, 0.0)));

    // Skein
    app.add_plugins(SkeinPlugin::default());

    app.insert_resource(WinitSettings::game());

    app.run()
}
