use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_flycam::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Component, PartialEq, Eq, Clone)]
struct Cell {
    alive: bool,
    state: u8,
    position: [usize; 3],
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.alive.hash(state);
        self.state.hash(state);
        self.position.hash(state)
    }
}

#[derive(Resource)]
struct Rules {
    survival: u8,
    born: u8,
    states: u8,
}

#[derive(Resource)]
struct CellBox<'a>(Vec<Vec<Vec<&'a Cell>>>);

const PLOT_SIZE: usize = 25;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .insert_resource(Rules {
            survival: 4,
            born: 4,
            states: 5,
        })
        .insert_resource(CellBox(vec![
            vec![
                vec![
                    &Cell {
                        alive: false,
                        state: 0,
                        position: [0, 0, 0]
                    };
                    PLOT_SIZE
                ];
                PLOT_SIZE
            ];
            PLOT_SIZE
        ]))
        .add_startup_system(setup_cell_neighbors)
        .add_startup_system(setup_cells)
        .add_system(kill_cells)
        .run();
}

fn setup_cells<'a>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    rules: Res<Rules>,
    cells: Res<CellBox>,
) {
    let handle = meshes.add(shape::Cube::new(0.5).into());
    for x in 0usize..PLOT_SIZE {
        for y in 0usize..PLOT_SIZE {
            for z in 0usize..PLOT_SIZE {
                commands.spawn((
                    Cell {
                        alive: false,
                        state: rules.states,
                        position: [x, y, z],
                    },
                    PbrBundle {
                        mesh: handle.clone(),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                ));
            }
        }
    }
}

fn setup_cell_neighbors(query: Query<&Cell>, mut commands: Commands) {}

fn moore_neighborhood(target: &[u32; 3], other: &[u32; 3]) -> bool {
    return if (target[0] - 1..target[0] + 1).contains(&other[0]) {
        true
    } else if (target[1] - 1..target[1] + 1).contains(&other[1]) {
        true
    } else if (target[2] - 1..target[2] + 1).contains(&other[2]) {
        true
    } else {
        false
    };
}

fn kill_cells(mut query: Query<(&mut Visibility, &Cell)>) {
    query
        .iter_mut()
        .filter(|(_, cell)| !cell.alive)
        .for_each(|(vis, _)| {
            let vis = vis.into_inner();
            *vis = Visibility::Hidden
        });
}
