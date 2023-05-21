#![allow(dead_code)]

use std::time::Duration;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

const PLOT_SIZE: usize = 25;
const ORBIT_BUTTON: MouseButton = MouseButton::Right;

#[derive(Component, Debug)]
struct Cell {
    alive: bool,
    state: u8,
}

#[derive(Resource)]
struct Rules {
    survival: u8,
    born: u8,
    states: u8,
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            survival: 4,
            born: 4,
            states: 5,
        }
    }
}

#[derive(Component)]
struct UiText;

#[derive(Resource)]
struct TickTimer {
    timer: Timer,
}

impl Default for TickTimer {
    fn default() -> Self {
        TickTimer {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
struct MainCamera {
    focus: Vec3,
    radius: f32,
}

impl Default for MainCamera {
    fn default() -> Self {
        MainCamera {
            focus: Vec3 {
                x: PLOT_SIZE as f32 / 2.0,
                y: PLOT_SIZE as f32 / 2.0,
                z: PLOT_SIZE as f32 / 2.0,
            },
            radius: PLOT_SIZE as f32 / 2.0,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Rules::default())
        .insert_resource(TickTimer::default())
        .add_startup_system(setup_main_camera)
        .add_startup_system(setup_cells)
        .add_system(main_camera)
        .add_system(color_cells)
        .add_system(tick)
        .run();
}

fn setup_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    rules: Res<Rules>,
) {
    let mesh_handle = meshes.add(shape::Cube::new(0.5).into());
    let mat_handle = mats.add(StandardMaterial {
        base_color: Color::rgb_u8(255, 255, 255),
        ..default()
    });
    for x in 0usize..PLOT_SIZE {
        for y in 0usize..PLOT_SIZE {
            for z in 0usize..PLOT_SIZE {
                commands.spawn((
                    Cell {
                        alive: false,
                        state: rules.states,
                    },
                    PbrBundle {
                        mesh: mesh_handle.clone(),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        visibility: Visibility::Visible,
                        material: mat_handle.clone(),
                        ..default()
                    },
                ));
            }
        }
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("~/.fonts/");
    commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Test".to_string(),
                    style: TextStyle {
                        font,
                        font_size: 13.0,
                        color: Color::WHITE,
                    },
                }],
                ..default()
            },
            ..default()
        })
        .insert(UiText);
}

fn update_ui(mut query: Query<&mut Text, With<UiText>>) {
    let text = query.single_mut().into_inner();
    text.sections[0].value = "Test".to_string();
}

fn setup_main_camera(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    let focus = Vec3::new(
        PLOT_SIZE as f32 / 2.0,
        PLOT_SIZE as f32 / 2.0,
        PLOT_SIZE as f32 / 2.0,
    );
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(
                PLOT_SIZE as f32 / 2.0,
                PLOT_SIZE as f32 / 2.0,
                0.0,
            ))
            .looking_at(focus, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.15, 0.45, 0.70)),
                ..default()
            },
            ..default()
        },
        MainCamera {
            radius: PLOT_SIZE as f32 * 2.0,
            ..default()
        },
    ));
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 4.5;
}

fn color_cells(
    query: Query<(&Handle<StandardMaterial>, &Cell)>,
    mut assets: ResMut<Assets<StandardMaterial>>,
) {
    for (material, cell) in query.iter() {
        *assets.get_mut(material).unwrap() = match cell.state {
            5 => Color::RED,
            4 => Color::YELLOW,
            3 => Color::ORANGE,
            2 => Color::GREEN,
            1 => Color::BLUE,
            0 => Color::VIOLET,
            _ => Color::WHITE,
        }
        .into();
    }
}

fn main_camera(
    window: Query<&Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut MainCamera, &mut Transform)>,
) {
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;

    if input_mouse.pressed(ORBIT_BUTTON) {
        ev_motion.iter().for_each(|ev| rotation_move += ev.delta);
    }
    ev_scroll.iter().for_each(|ev| scroll += ev.y);

    let window_res = &window.single().resolution;
    let window_size = Vec2::new(window_res.height(), window_res.width());

    for (mut camera, mut transform) in query.iter_mut() {
        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let delta_x = rotation_move.x / window_size.x * std::f32::consts::PI * 2.0;
            // let delta_y = rotation_move.y / window_size.y * std::f32::consts::PI * 2.0;
            let yaw = Quat::from_rotation_y(-delta_x);
            // let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation *= yaw;
            // transform.rotation *= pitch;
        } else if scroll.abs() > 0.0 {
            any = true;
            camera.radius -= scroll * camera.radius * 0.2;
            camera.radius = f32::max(camera.radius, 0.05);
        }

        if any {
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                camera.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, camera.radius));
        }
    }
    ev_motion.clear();
}

fn tick(
    mut timer: ResMut<TickTimer>,
    time: Res<Time>,
    mut query: Query<(&mut Visibility, &mut Cell)>,
) {
    timer.timer.tick(time.delta());
    if !timer.timer.finished() {
        return;
    }

    query
        .iter_mut()
        .filter(|(_, cell)| !cell.alive)
        .for_each(|(vis, cell)| match cell.state {
            2..=u8::MAX => cell.into_inner().state -= 1,
            0..=1 => {
                *vis.into_inner() = Visibility::Hidden;
            }
        });
}
