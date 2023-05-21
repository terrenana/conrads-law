use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Cell {
    pub alive: bool,
    pub state: u8,
}

#[derive(Resource)]
pub struct Rules {
    pub survival: u8,
    pub born: u8,
    pub states: u8,
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

#[derive(Resource)]
struct MaterialHandles {
    default_color: Handle<StandardMaterial>,
    state_colors: Vec<Handle<StandardMaterial>>,
}

#[derive(Resource)]
pub struct TickTimer {
    pub timer: Timer,
    pub ticks: u64,
}

impl Default for TickTimer {
    fn default() -> Self {
        TickTimer {
            timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
            ticks: 0,
        }
    }
}

fn setup_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mats: Res<MaterialHandles>,
    rules: Res<Rules>,
) {
    let mesh_handle = meshes.add(shape::Cube::new(1.0).into());
    for x in 0usize..crate::PLOT_SIZE {
        for y in 0usize..crate::PLOT_SIZE {
            for z in 0usize..crate::PLOT_SIZE {
                commands.spawn((
                    Cell {
                        alive: true,
                        state: rules.states,
                    },
                    PbrBundle {
                        mesh: mesh_handle.clone(),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        visibility: Visibility::Visible,
                        material: mats.default_color.clone(),
                        ..default()
                    },
                ));
            }
        }
    }
}

fn setup_cell_materials(mut commands: Commands, mut mats: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(MaterialHandles {
        default_color: mats.add(StandardMaterial {
            base_color: crate::STATE_DEFAULT_COLOR,
            ..default()
        }),
        state_colors: crate::STATE_COLORS
            .iter()
            .map(|color| {
                mats.add(StandardMaterial {
                    base_color: *color,
                    ..default()
                })
            })
            .collect(),
    });
}

fn color_cells(
    mut query: Query<(&mut Handle<StandardMaterial>, &Cell)>,
    mats: Res<MaterialHandles>,
) {
    for (material, cell) in query.iter_mut() {
        *material.into_inner() = mats
            .state_colors
            .get(cell.state as usize)
            .unwrap_or(&mats.default_color)
            .clone();
    }
}

fn tick(
    mut timer: ResMut<TickTimer>,
    time: Res<Time>,
    mut query: Query<(&mut Visibility, &mut Cell)>,
) {
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
    }
    timer.ticks += 1;

    query
        .iter_mut()
        .enumerate()
        .filter(|(ix, _)| ix % 2 == 0)
        .for_each(|(_, (_, cell))| cell.into_inner().alive = false);

    query
        .iter_mut()
        .filter(|(_, cell)| !cell.alive)
        .for_each(|(vis, cell)| match cell.state {
            2..=u8::MAX => cell.into_inner().state -= 1,
            0..=1 => {
                cell.into_inner().state = 0;
                *vis.into_inner() = Visibility::Hidden;
            }
        });
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Rules::default())
            .insert_resource(TickTimer::default())
            .add_startup_system(setup_cells)
            .add_startup_system(setup_cell_materials.in_base_set(StartupSet::PreStartup))
            .add_system(color_cells)
            .add_system(tick);
    }
}
