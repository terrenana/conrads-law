use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::simulation::SimulationState;


pub struct UiPlugin;

#[derive(Component)]
pub struct UiText;

#[derive(Resource)]
pub struct Settings {
    pub paused: bool
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            paused: false
        }
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup_text)
            .add_system(update_fps)
            .add_system(update_ticks)
            .add_system(update_settings)
            .add_system(handle_debug_keys)
            .insert_resource(Settings::default());
    }
}

fn handle_debug_keys(mut query: Query<&mut Visibility, With<UiText>>, keys: Res<Input<KeyCode>>, mut settings: ResMut<Settings>) {
    if keys.just_pressed(KeyCode::Escape) {
        let text = query.single_mut();
        if let Visibility::Visible = *text {
            *text.into_inner() = Visibility::Hidden
        } else {
            *text.into_inner() = Visibility::Visible;
        }
    }
    if keys.just_pressed(KeyCode::Space) {
        settings.paused = !settings.paused;
    }
}

fn setup_text(mut commands: Commands, server: Res<AssetServer>) {
    let style = TextStyle {
        font: server.load("font.ttf"),
        font_size: 25.0,
        color: Color::WHITE,
    };

    commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection::new("FPS\n", style.clone()),
                    TextSection::new("TICKS\n", style.clone()),
                    TextSection::new("", style.clone()),
 //                   TextSection::new("STATES: \n", style),
                ],
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        })
        .insert(UiText);
}

fn update_fps(diag: Res<Diagnostics>, mut query: Query<&mut Text, With<UiText>>) {
    let mut text = query.single_mut();

    if let Some(fps) = diag.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.sections[0].value = format!("FPS: {value}")
        }
    }
}

fn update_ticks(
    state: Res<SimulationState>,
    mut query: Query<&mut Text, With<UiText>>,
) {
    let mut text = query.single_mut();

    let value = state.into_inner().ticks;

    text.sections[1].value = format!("\nTICKS: {value}");
}

fn update_settings(settings: Res<Settings>, mut query: Query<&mut Text, With<UiText>>) {
    let mut text = query.single_mut();

    if settings.paused {
        text.sections[2].value = format!("\nPAUSED");
    } else {
        text.sections[2].value = format!("");
    }
}

/*fn update_states(
    mut text: Query<&mut Text, With<UiText>>,
    state: Res<SimulationState>,
    rules: Res<crate::rules::Rules>,
) {
    let mut text = text.single_mut();   
    let mut value = String::new();

    let cells = state.current.clone();

    text.sections[3].value = format!("\nSTATES: {value}");
}*/
