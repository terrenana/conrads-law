use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct UiPlugin;

#[derive(Component)]
struct UiText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup)
            .add_system(update_fps)
            .add_system(update_ticks)
            .add_system(update_dead)
            .add_system(update_states);
    }
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let style = TextStyle {
        font: server.load("font.ttf"),
        font_size: 25.0,
        color: Color::WHITE,
    };

    commands
        .spawn(TextBundle::from_sections([
            TextSection::new("FPS\n", style.clone()),
            TextSection::new("TICKS\n", style.clone()),
            TextSection::new("DEAD\n", style.clone()),
            TextSection::new("STATES: \n", style),
        ]))
        .insert(UiText);
}

fn update_fps(diag: Res<Diagnostics>, mut query: Query<&mut Text, With<UiText>>) {
    let mut text = query.single_mut();

    if let Some(fps) = diag.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.sections[0].value = format!("FPS: {value}\n")
        }
    }
}

fn update_ticks(
    timer: Res<crate::simulation::TickTimer>,
    mut query: Query<&mut Text, With<UiText>>,
) {
    let mut text = query.single_mut();

    let value = timer.ticks;

    text.sections[1].value = format!("TICKS: {value}\n");
}

fn update_dead(mut text: Query<&mut Text, With<UiText>>, cells: Query<&crate::simulation::Cell>) {
    let mut text = text.single_mut();

    let value = cells.iter().filter(|cell| !cell.alive).count();

    text.sections[2].value = format!("DEAD: {value}\n");
}

fn update_states(
    mut text: Query<&mut Text, With<UiText>>,
    cells: Query<&crate::simulation::Cell>,
    rules: Res<crate::simulation::Rules>,
) {
    let mut text = text.single_mut();

    let mut value = "".to_string();

    for state in 0..=rules.states {
        value.push_str(&format!("\n{state}: "));
        value.push_str(
            &cells
                .iter()
                .filter(|cell| cell.state == state)
                .count()
                .to_string(),
        );
    }
    text.sections[3].value = format!("STATES: {value}\n");
}
