use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::simulation::{CellColorMode, CellState, TickTimer, Toggleables};

pub struct UiPlugin;

#[derive(Component)]
pub struct UiText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup)
            .add_system(update_fps)
            .add_system(update_ticks)
            .add_system(update_debug)
            .add_system(update_states)
            .add_system(handle_debug_keys);
    }
}

fn handle_debug_keys(mut query: Query<&mut Visibility, With<UiText>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        let text = query.single_mut();
        if let Visibility::Visible = *text {
            *text.into_inner() = Visibility::Hidden
        } else {
            *text.into_inner() = Visibility::Visible;
        }
    }
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
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
                    TextSection::new("STATES: \n", style),
                ],
                ..default()
            },
            visibility: Visibility::Hidden,
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
    timer: Res<crate::simulation::TickTimer>,
    mut query: Query<&mut Text, With<UiText>>,
) {
    let mut text = query.single_mut();

    let value = timer.ticks;

    text.sections[1].value = format!("\nTICKS: {value}");
}

fn update_debug(
    mut text: Query<&mut Text, With<UiText>>,
    debug: Res<Toggleables>,
    timer: Res<TickTimer>,
) {
    let mut text = text.single_mut();
    let mut value = "".to_string();

    if debug.suppress_death {
        value.push_str("\nDEATH SUPPRESSED");
    } else {
        value.push('\n');
    }
    if timer.timer.paused() {
        value.push_str("\nPAUSED");
    } else {
        value.push('\n');
    }
    value.push_str(match debug.cell_color_mode {
        CellColorMode::State => "\nC_MODE: state",
        CellColorMode::Dist => "\nC_MODE: dist",
    });
    text.sections[2].value = value;
}

fn update_states(
    mut text: Query<&mut Text, With<UiText>>,
    cells: Query<&crate::simulation::Cell>,
    rules: Res<crate::simulation::Rules>,
) {
    let mut text = text.single_mut();

    let mut value = String::new();

    let dead = cells
        .iter()
        .filter(|cell| *cell.state.lock().unwrap() == CellState::Dead)
        .count();

    value.push_str(&format!("\nDEAD: {dead}"));
    for state in 1..rules.states - 1 {
        let statenum = cells
            .iter()
            .filter(|cell| *cell.state.lock().unwrap() == CellState::Dying(state))
            .count();
        value.push_str(&format!("\n{state}: {statenum}"));
    }

    let alive = cells
        .iter()
        .filter(|cell| *cell.state.lock().unwrap() == CellState::Alive)
        .count();

    value.push_str(&format!("\nALIVE: {alive}"));
    text.sections[3].value = format!("\nSTATES: {value}");
}
