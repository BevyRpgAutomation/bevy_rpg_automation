use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use game_camera::states::PlayerCameraMode;

use crate::components::{CurrentPlayerCameraModeText, FpsText};

pub fn spawn_debug_hud(mut commands: Commands) {
    commands
        .spawn(
            (Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            }),
        )
        .with_children(|parent| {
            parent.spawn(Text::new(
                "Press V to change player camera mode. Currently:",
            ));
            parent.spawn((Text::new(""), CurrentPlayerCameraModeText));
            parent.spawn((Text::new(""), FpsText));
        });
}

pub fn update_current_player_camera_mode_text(
    current_player_camera_mode: Res<State<PlayerCameraMode>>,
    mut current_player_camera_mode_text_query: Query<&mut Text, With<CurrentPlayerCameraModeText>>,
) {
    if current_player_camera_mode.is_changed() {
        let Ok(mut current_player_camera_mode_text) =
            current_player_camera_mode_text_query.single_mut()
        else {
            return;
        };
        *current_player_camera_mode_text = Text(current_player_camera_mode.get().to_string());
    }
}

pub fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_query: Query<&mut Text, With<FpsText>>,
) {
    let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) else {
        warn!("Failed to retrieve fps from diagnostics store");
        return;
    };

    let Some(fps_average) = fps_diagnostic.average() else {
        return;
    };

    let Ok(mut fps_text) = fps_text_query.single_mut() else {
        return;
    };

    *fps_text = Text(format!("FPS(avg): {}", fps_average.to_string()));
}
