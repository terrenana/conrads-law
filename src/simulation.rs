use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::helpers::{self, c32, CVec3};
use crate::rules::*;

use bevy::{prelude::*, utils::hashbrown::HashMap};

#[derive(Component, Debug)]
pub struct Cell {
    pub state: Arc<Mutex<CellState>>,
    pub buffered_state: Mutex<Option<CellState>>,
    pub neighbors: Vec<Arc<Mutex<CellState>>>,
}
#[derive(Component, Debug)]
struct Position(CVec3);
#[derive(Resource)]
pub struct Toggleables {
    pub suppress_death: bool,
    pub cell_color_mode: CellColorMode,
    pub step: bool,
}

impl Default for Toggleables {
    fn default() -> Self {
        Toggleables {
            suppress_death: false,
            cell_color_mode: CellColorMode::State,
            step: false,
        }
    }
}

pub enum CellColorMode {
    State,
    Dist,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellState {
    Alive,
    Dying(u8),
    Dead,
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
            timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
            ticks: 0,
        }
    }
}

fn setup_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mats: Res<MaterialHandles>,
) {
    let mesh_handle = meshes.add(shape::Cube::new(1.0).into());
    for x in 0..=crate::PLOT_SIZE {
        for y in 0..=crate::PLOT_SIZE {
            for z in 0..=crate::PLOT_SIZE {
                commands.spawn((
                    Cell {
                        state: Arc::new(Mutex::new(CellState::Dead)),
                        buffered_state: Mutex::new(None),
                        neighbors: Vec::new(),
                    },
                    Position(CVec3::new(x, y, z)),
                    PbrBundle {
                        mesh: mesh_handle.clone(),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        visibility: Visibility::Hidden,
                        material: mats.default_color.clone(),
                        ..default()
                    },
                ));
            }
        }
    }
}

fn setup_cell_matrix(mut cells: Query<&mut Cell>, positions: Query<&Position>, rules: Res<Rules>) {
    let mut pos_cell_map: HashMap<CVec3, Mut<Cell>> = HashMap::new();
    cells
        .iter_mut()
        .zip(positions.iter())
        .for_each(|(cell, pos)| {
            pos_cell_map.insert(pos.0, cell);
        });
    for pos in positions.iter().map(|pos| pos.0) {
        let mut neighbors: Vec<Arc<Mutex<CellState>>> = Vec::new();
        for offset in rules.neighborhood_matrix.iter() {
            let cell = pos_cell_map.get(&(pos + *offset));
            if let Some(e) = cell {
                neighbors.push(e.state.clone());
            };
        }
        pos_cell_map.get_mut(&pos).unwrap().neighbors = neighbors;
    }
}

fn setup_cell_materials(mut commands: Commands, mut mats: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(MaterialHandles {
        default_color: mats.add(StandardMaterial {
            base_color: crate::STATE_DEFAULT_COLOR,
            ..default()
        }),
        state_colors: crate::GREEN_TO_RED
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
    mut query: Query<(&mut Handle<StandardMaterial>, &Cell, &mut Visibility)>,
    mats: Res<MaterialHandles>,
    rules: Res<Rules>,
    color_rule: Res<Toggleables>,
) {
    match color_rule.cell_color_mode {
        CellColorMode::State => {
            for (material, cell, vis) in query.iter_mut() {
                *material.into_inner() = mats
                    .state_colors
                    .get(match *cell.state.lock().unwrap() {
                        CellState::Alive => {
                            *vis.into_inner() = Visibility::Visible;
                            rules.states
                        }
                        CellState::Dying(x) => x,
                        CellState::Dead => 0,
                    } as usize)
                    .unwrap_or(&mats.default_color)
                    .clone();
            }
        }
        CellColorMode::Dist => {}
    }
}

fn tick(
    mut timer: ResMut<TickTimer>,
    time: Res<Time>,
    mut query: Query<(&mut Visibility, &mut Cell)>,
    rules: Res<Rules>,
    mut debug: ResMut<Toggleables>,
) {
    if !debug.step {
        timer.timer.tick(time.delta());
        if !timer.timer.just_finished() {
            return;
        }
        timer.ticks += 1;
    } else {
        debug.step = false;
    }

    query.iter_mut().for_each(|(vis, cell)| {
        let mut cell_state_mutex_lock = cell.state.lock().unwrap();
        let mut cell_buffer_mutex_lock = cell.buffered_state.lock().unwrap();
        if let Some(state) = *cell_buffer_mutex_lock {
            *cell_state_mutex_lock = state;
            *cell_buffer_mutex_lock = None;
        }
        match *cell_state_mutex_lock {
            CellState::Alive => {
                let alive_neighbor_ct = cell
                    .neighbors
                    .iter()
                    .filter(|&n| *n.lock().unwrap() == CellState::Alive)
                    .count() as u8;

                if !rules.survival.has_match(alive_neighbor_ct) && !debug.suppress_death {
                    *cell_buffer_mutex_lock = Some(CellState::Dying(rules.states - 1));
                }
            }
            CellState::Dying(x) => {
                match x {
                    2..=u8::MAX => *cell_state_mutex_lock = CellState::Dying(x - 1),
                    0..=1 => {
                        *vis.into_inner() = Visibility::Hidden;
                        *cell_buffer_mutex_lock = Some(CellState::Dead);
                    }
                };
            }
            CellState::Dead => {
                *vis.into_inner() = Visibility::Hidden;
                let alive_neighbor_ct = cell
                    .neighbors
                    .iter()
                    .filter(|&n| *n.lock().unwrap() == CellState::Alive)
                    .count() as u8;
                if rules.born.has_match(alive_neighbor_ct) {
                    *cell_buffer_mutex_lock = Some(CellState::Alive);
                }
            }
        }
    });
}

fn spawn_cell_noise(
    query: Query<(&Cell, &Position)>,
    keys: Res<Input<KeyCode>>,
    mut debug: ResMut<Toggleables>,
    timer: Res<TickTimer>,
) {
    if keys.just_pressed(KeyCode::Return) {
        let mut rand = rand::thread_rng();
        for (cell, pos) in query.iter() {
            if helpers::noise_func(pos.0, 10.0) {
                if rand.gen_range(1..=2) == 2 {
                    *cell.buffered_state.lock().unwrap() = Some(CellState::Alive);
                }
            }
        }
        if timer.timer.paused() {
            debug.step = true;
        }
    }
}

fn handle_keys(
    keys: Res<Input<KeyCode>>,
    mut timer: ResMut<TickTimer>,
    mut toggleables: ResMut<Toggleables>,
    mut query: Query<&Cell>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match timer.timer.paused() {
            true => timer.timer.unpause(),
            false => timer.timer.pause(),
        }
    } else if keys.just_pressed(KeyCode::M) {
        toggleables.cell_color_mode = match toggleables.cell_color_mode {
            CellColorMode::State => CellColorMode::Dist,
            CellColorMode::Dist => CellColorMode::State,
        };
    } else if keys.just_pressed(KeyCode::Back) {
        query
            .iter_mut()
            .for_each(|cell| *cell.state.lock().unwrap() = CellState::Dead);
    } else if keys.just_pressed(KeyCode::S) {
        toggleables.suppress_death = !toggleables.suppress_death;
    } else if keys.just_pressed(KeyCode::Right) {
        toggleables.step = true;
        timer.ticks += 1;
    }
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Rules::_445())
            .insert_resource(TickTimer::default())
            .insert_resource(Toggleables::default())
            .add_startup_system(setup_cells)
            .add_startup_system(setup_cell_materials.in_base_set(StartupSet::PreStartup))
            .add_startup_system(setup_cell_matrix.in_base_set(StartupSet::PostStartup))
            .add_system(spawn_cell_noise)
            .add_system(color_cells)
            .add_system(handle_keys)
            .add_system(tick);
    }
}
