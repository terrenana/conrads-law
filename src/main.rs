#![allow(dead_code)]

mod rules;
mod simulation;
mod display;
mod user;
use bevy::prelude::*;

const ORBIT_BUTTON: MouseButton = MouseButton::Left;
const PLOT_SIZE: usize = 64;
const GREEN_TO_RED: [Color; 6] = [
    Color::rgb(0.6, 1.0, 0.0),
    Color::rgb(0.9, 1.0, 0.0),
    Color::rgb(1.0, 0.9, 0.0),
    Color::rgb(1.0, 0.6, 0.0),
    Color::rgb(1.0, 0.3, 0.0),
    Color::RED,
];
const STATE_DEFAULT_COLOR: Color = Color::RED;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(simulation::SimulationPlugin)
        .add_plugin(display::DisplayPlugin)
        .add_plugin(user::camera::CameraPlugin)
        .add_plugin(user::ui::UiPlugin)
        .run();

}

