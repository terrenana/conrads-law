use bevy::math::ivec3;
use bevy::prelude::*;
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

pub fn moore(target: IVec3, other: IVec3) -> bool {
    for moore_offset in MOORE_NEIGHBORHOOD.into_iter() {
        if add_ivec3(target, moore_offset) == other {
            return true;
        }
    }
    return false;
}

pub fn distance(target: IVec3, other: IVec3) -> i32 {
    target.as_vec3().distance(other.as_vec3()) as i32
}

pub fn add_ivec3(a: IVec3, b: IVec3) -> IVec3 {
    IVec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

#[test]
fn test_moore() {
    for ivec3 in MOORE_NEIGHBORHOOD.into_iter() {
        assert!(moore(
            IVec3::new(10, 10, 10),
            add_ivec3(IVec3::new(10, 10, 10), ivec3)
        ))
    }
}
