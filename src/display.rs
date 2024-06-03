use bevy::prelude::*;

use crate::{rules::Rules, simulation::{SimulationState, _CellState}, PLOT_SIZE};

#[derive(Resource)]
struct MaterialHandles {
    default_color: Handle<StandardMaterial>,
    state_colors: Vec<Handle<StandardMaterial>>,
}

#[derive(Component)]
struct Position(Vec2);

pub struct DisplayPlugin;

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

fn setup_cell_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mats: Res<MaterialHandles>) {
    let mesh_handle = meshes.add(shape::Cube::new(1.0).into());
    for x in 0..PLOT_SIZE {
        for y in 0..PLOT_SIZE {
            commands.spawn((
                    PbrBundle {
                        mesh: mesh_handle.clone(),
                        transform: Transform::from_xyz(x as f32, y as f32, 1.0),
                        visibility: Visibility::Visible,
                        material: mats.default_color.clone(),
                        ..default()
                    },
                    Position(Vec2::new(x as f32, y as f32))
            ));
        }
    }
}

fn update(state: Res<SimulationState>, mut query: Query<(&mut Visibility, &Position, &mut Handle<StandardMaterial>)>, mats: Res<MaterialHandles>, rules: Res<Rules>) {
    let state = state.current.clone();

    for (mut vis, pos, mut mat) in query.iter_mut() {
        let cellstate = state.get(pos.0.x.round() as isize, pos.0.y.round() as isize);
        match cellstate {
            _CellState::Alive => {
                *vis = Visibility::Visible; 
                *mat = mats.state_colors.get(rules.states).unwrap_or(&mats.default_color).clone();
            },
            _CellState::Dying(x) => {
                *mat = mats.state_colors.get(*x).unwrap_or(&mats.default_color).clone();
            },
            _CellState::Dead => {
                *vis = Visibility::Hidden;
            },
        }
    }
}


impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_cell_materials.in_base_set(StartupSet::PreStartup))
            .add_startup_system(setup_cell_meshes.in_base_set(StartupSet::PostStartup))
            .add_system(update.in_base_set(CoreSet::PostUpdate));
    }
}
