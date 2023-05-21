mod simulation;
mod user;

use bevy::prelude::*;

const ORBIT_BUTTON: MouseButton = MouseButton::Right;
const PLOT_SIZE: usize = 10;
const STATE_COLORS: [Color; 6] = [
    Color::rgb(1.0, 1.0, 1.0),
    Color::rgb(1.0, 0.8, 0.8),
    Color::rgb(1.0, 0.6, 0.6),
    Color::rgb(1.0, 0.5, 0.5),
    Color::rgb(1.0, 0.4, 0.4),
    Color::RED,
];
const STATE_DEFAULT_COLOR: Color = Color::WHITE;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(simulation::SimulationPlugin)
        .add_plugin(user::camera::CameraPlugin)
        .add_plugin(user::ui::UiPlugin)
        .run();
}
