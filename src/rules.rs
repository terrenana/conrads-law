use std::ops::RangeInclusive;

use bevy::{math::ivec3, prelude::*};

pub static MOORE_NEIGHBORHOOD: [(isize, isize, isize); 26] = [
    (-1, -1, -1),
    (0, -1, -1),
    (1, -1, -1),
    (-1, 0, -1),
    (0, 0, -1),
    (1, 0, -1),
    (-1, 1, -1),
    (0, 1, -1),
    (1, 1, -1),
    (-1, -1, 0),
    (0, -1, 0),
    (1, -1, 0),
    (-1, 0, 0),
    (1, 0, 0),
    (-1, 1, 0),
    (0, 1, 0),
    (1, 1, 0),
    (-1, -1, 1),
    (0, -1, 1),
    (1, -1, 1),
    (-1, 0, 1),
    (0, 0, 1),
    (1, 0, 1),
    (-1, 1, 1),
    (0, 1, 1),
    (1, 1, 1),
];

const MOORE_NEIGHBORHOOD_2D: [(isize, isize); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];
pub static VN_NEIGHBORHOOD: [(isize, isize, isize); 6] = [
    (1, 0, 0),
    (0, 1, 0),
    (0, 0, 1),
    (-1, 0, 0),
    (0, -1, 0),
    (0, 0, -1),
];
#[derive(Resource)]
pub struct Rules {
    pub survival: Rule,
    pub born: Rule,
    pub states: usize,
    pub neighborhood_matrix: Vec<(isize, isize)>,
}

pub enum Rule {
    Single(u8),
    Range(RangeInclusive<u8>),
    Multi(Vec<Rule>),
}

impl Rule {
    pub fn has_match(&self, match_target: u8) -> bool {
        match self {
            Rule::Single(n) => {
                match_target == *n
            },
            Rule::Range(range) => range.contains(&match_target),
            Rule::Multi(vec) => {
                for rule in vec {
                    if rule.has_match(match_target) {
                        return true;
                    }
                }
                return false;
            }
        }
    }
}

impl Rules {
    pub fn _2dgol() -> Self {
        Rules {
            survival: Rule::Range(2..=3),
            born: Rule::Single(3),
            states: 5,
            neighborhood_matrix: MOORE_NEIGHBORHOOD_2D.to_vec()
        }
    }
    /*
    pub fn _445() -> Self {
        Rules {
            survival: Rule::Single(4),
            born: Rule::Single(4),
            states: 5,
            neighborhood_matrix: MOORE_NEIGHBORHOOD.to_vec(),
        }
    }
    pub fn _455() -> Self {
        Rules {
            survival: Rule::Range(4..=5),
            born: Rule::Single(5),
            states: 2,
            neighborhood_matrix: MOORE_NEIGHBORHOOD.to_vec(),
        }
    }
    pub fn clouds() -> Self {
        Rules {
            survival: Rule::Range(13..=26),
            born: Rule::Multi(vec![Rule::Range(13..=14), Rule::Range(17..=19)]),
            states: 2,
            neighborhood_matrix: MOORE_NEIGHBORHOOD.to_vec(),
        }
    }
    pub fn _678_678() -> Self {
        Rules {
            survival: Rule::Range(6..=8),
            born: Rule::Range(6..=8),
            states: 3,
            neighborhood_matrix: MOORE_NEIGHBORHOOD.to_vec(),
        }
    }
    pub fn amoeba() -> Self {
        Rules {
            survival: Rule::Range(9..=26),
            born: Rule::Multi(vec![
                Rule::Range(6..=7),
                Rule::Range(12..=13),
                Rule::Single(15),
            ]),
            states: 5,
            neighborhood_matrix: MOORE_NEIGHBORHOOD.to_vec(),
        }
    }
    pub fn builder() -> Self {
        Rules {
            survival: Rule::Multi(vec![Rule::Single(2), Rule::Single(6), Rule::Single(9)]),
            born: Rule::Multi(vec![Rule::Single(4), Rule::Single(6), Rule::Range(8..=9)]),
            states: 10,
            neighborhood_matrix: MOORE_NEIGHBORHOOD.to_vec(),
        }
    }
    pub fn crystal() -> Self {
        Rules {
            survival: Rule::Range(0..=6),
            born: Rule::Multi(vec![Rule::Single(1), Rule::Single(3)]),
            states: 2,
            neighborhood_matrix: VN_NEIGHBORHOOD.to_vec(),
        }
    }*/
}

impl Default for Rules {
    fn default() -> Self {
        Rules::_2dgol()
    }
}
