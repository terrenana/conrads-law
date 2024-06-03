use core::fmt;
use std::fmt::Display;
use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use crate::rules::Rules;
use crate::user::ui::Settings;
use crate::PLOT_SIZE;


#[derive(Clone, Debug)]
pub enum _CellState {
    Alive,
    Dying(usize),
    Dead
}

impl Display for _CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            _CellState::Alive => write!(f, "⬛"),
            _CellState::Dying(x) => write!(f, "{x}"),
            _CellState::Dead => write!(f, " "),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CellArray {
    pub data: Vec<Vec<_CellState>>,
    dim: usize
}

impl Display for CellArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp = String::new();
        self.data.iter().for_each(|c| {
            c.iter().for_each(|r| {
                disp += &format!("{} ", r);
            });
            disp += "\n";
        });
        write!(f, "{disp}")
    }
}

impl CellArray {
    fn new(dim: usize) -> Self {
        CellArray{
            data: vec![vec![_CellState::Dead; dim]; dim],
            dim
        }   
    }
    pub fn get(&self, i: isize, j: isize) -> &_CellState {
        let dim = self.dim as isize;
        let x = ((i % dim + dim) % dim) as usize;
        let y = ((j % dim + dim) % dim) as usize;
        &self.data[x][y]
    }
}

#[derive(Debug, Clone, Resource)]
pub struct SimulationState {
    pub current: CellArray,
    buffer: CellArray,
    pub ticks: u32
}

unsafe impl Send for SimulationState {}
unsafe impl Sync for SimulationState {}

impl Display for SimulationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.current)
    }
}

fn setup(mut commands: Commands) {
    let mut state = SimulationState {
        current: CellArray::new(PLOT_SIZE),
        buffer: CellArray::new(PLOT_SIZE),
        ticks: 0
    };
    state.buffer.data[20][1] = _CellState::Alive;
    state.buffer.data[20][2] = _CellState::Alive;
    state.buffer.data[20][3] = _CellState::Alive;

    state.buffer.data[25][26] = _CellState::Alive;
    state.buffer.data[26][25] = _CellState::Alive;
    state.buffer.data[27][25] = _CellState::Alive;
    state.buffer.data[27][26] = _CellState::Alive;
    state.buffer.data[27][27] = _CellState::Alive;
    commands.insert_resource(state);
    commands.insert_resource(Rules::_2dgol());
}


fn tick(state: ResMut<SimulationState>, rules: Res<Rules>, settings: Res<Settings>) {
    if settings.paused {
        return;
    }
    let state = state.into_inner();
    state.ticks += 1;
    //println!("{}", state);
    state.current = state.buffer.clone();
    state.buffer = CellArray::new(PLOT_SIZE);

    for (i, row) in state.buffer.data.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            let mut alive_ct = 0;
            for (x, y) in rules.neighborhood_matrix.iter() {
                match state.current.get(i as isize - x, j as isize - y) {
                    _CellState::Alive => alive_ct += 1,
                    _ => ()
                }
            }
            *cell = match state.current.get(i as isize, j as isize) {
                _CellState::Alive => {
                    if rules.survival.has_match(alive_ct) {
                        _CellState::Alive
                    } else {
                        if rules.states == 2 {
                            _CellState::Dead
                        } else {
                            _CellState::Dying(rules.states - 1)
                        }
                    }
                },
                _CellState::Dying(n) => {
                    if n == &1usize {
                        _CellState::Dead
                    } else if rules.born.has_match(alive_ct) {
                        _CellState::Alive
                    } else {
                        _CellState::Dying(n-1)
                    }
                },
                _CellState::Dead => {
                    if rules.born.has_match(alive_ct) {
                        _CellState::Alive
                    } else {
                        _CellState::Dead
                    }
                }
            }
        }
    }
}


pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup.in_base_set(StartupSet::PreStartup))
            .add_system(tick
                        .run_if(on_timer(Duration::from_secs(1)))
                        .in_base_set(CoreSet::Update));
    }
}
