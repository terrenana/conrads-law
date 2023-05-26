use std::ops::RangeInclusive;

use bevy::{math::ivec3, prelude::*};

pub static MOORE_NEIGHBORHOOD: [IVec3; 26] = [
    ivec3(-1, -1, -1),
    ivec3(0, -1, -1),
    ivec3(1, -1, -1),
    ivec3(-1, 0, -1),
    ivec3(0, 0, -1),
    ivec3(1, 0, -1),
    ivec3(-1, 1, -1),
    ivec3(0, 1, -1),
    ivec3(1, 1, -1),
    ivec3(-1, -1, 0),
    ivec3(0, -1, 0),
    ivec3(1, -1, 0),
    ivec3(-1, 0, 0),
    ivec3(1, 0, 0),
    ivec3(-1, 1, 0),
    ivec3(0, 1, 0),
    ivec3(1, 1, 0),
    ivec3(-1, -1, 1),
    ivec3(0, -1, 1),
    ivec3(1, -1, 1),
    ivec3(-1, 0, 1),
    ivec3(0, 0, 1),
    ivec3(1, 0, 1),
    ivec3(-1, 1, 1),
    ivec3(0, 1, 1),
    ivec3(1, 1, 1),
];

pub static VN_NEIGHBORHOOD: [IVec3; 6] = [
    ivec3(1, 0, 0),
    ivec3(0, 1, 0),
    ivec3(0, 0, 1),
    ivec3(-1, 0, 0),
    ivec3(0, -1, 0),
    ivec3(0, 0, -1),
];
#[derive(Resource)]
pub struct Rules {
    pub survival: Rule,
    pub born: Rule,
    pub states: u8,
    pub neighborhood_matrix: Vec<IVec3>,
}

pub enum Rule {
    Single(u8),
    Range(RangeInclusive<u8>),
    Multi(Vec<Rule>),
}

impl Rule {
    pub fn has_match(&self, match_target: u8) -> bool {
        match self {
            Rule::Single(n) => match_target == *n,
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
    pub fn _445() -> Self {
        Rules {
            survival: Rule::Single(3),
            born: Rule::Single(3),
            states: 5,
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
    }
}

impl Default for Rules {
    fn default() -> Self {
        Rules::_445()
    }
}
