use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

#[derive(Component)]
struct MainCamera {
    focus: Vec3,
    radius: f32,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(update);
    }
}

impl Default for MainCamera {
    fn default() -> Self {
        MainCamera {
            focus: Vec3 {
                x: crate::PLOT_SIZE as f32 / 2.0,
                y: crate::PLOT_SIZE as f32 / 2.0,
                z: crate::PLOT_SIZE as f32 / 2.0,
            },
            radius: crate::PLOT_SIZE as f32 / 2.0,
        }
    }
}

fn setup(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    let focus = Vec3::new(
        crate::PLOT_SIZE as f32 / 2.0,
        crate::PLOT_SIZE as f32 / 2.0,
        crate::PLOT_SIZE as f32 / 2.0,
    );
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(
                crate::PLOT_SIZE as f32 / 2.0,
                crate::PLOT_SIZE as f32 / 2.0,
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
            radius: crate::PLOT_SIZE as f32 * 2.0,
            ..default()
        },
    ));
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 1.0;
}
fn update(
    window: Query<&Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut MainCamera, &mut Transform)>,
) {
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;

    if input_mouse.pressed(crate::ORBIT_BUTTON) {
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
