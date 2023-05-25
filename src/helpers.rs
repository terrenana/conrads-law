use std::ops::Add;

use bevy::{math::ivec3, prelude::*};

use crate::PLOT_SIZE;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd, Debug, Eq, Hash, Clone, Copy)]
pub struct c32 {
    v: i32,
}

impl c32 {
    pub fn new(v: i32) -> Self {
        c32 {
            v: {
                if v > PLOT_SIZE as i32 {
                    (v - 1) - PLOT_SIZE as i32
                } else if v < 0 {
                    PLOT_SIZE as i32 + (v + 1)
                } else {
                    v
                }
            },
        }
    }
    fn i32(self) -> i32 {
        self.v
    }
}

impl Add for c32 {
    type Output = c32;
    fn add(self, rhs: Self) -> Self::Output {
        c32::new(self.v + rhs.v)
    }
}
impl Add<u32> for c32 {
    type Output = c32;
    fn add(self, rhs: u32) -> Self::Output {
        c32::new(self.v + rhs as i32)
    }
}

impl Add<i32> for c32 {
    type Output = c32;
    fn add(self, rhs: i32) -> Self::Output {
        c32::new(self.v as i32 + rhs)
    }
}
#[test]
fn test_add_c32() {
    assert!(c32::new(PLOT_SIZE as i32) + c32::new(1) == c32::new(0));

    // assert!(CVec3::new(PLOT_SIZE, PLOT_SIZE, PLOT_SIZE) + ivec3(1, 1, 1) == CVec3::new(1, 1, 1));
    assert!(CVec3::new(0, 0, 0) + ivec3(-1, -1, -1) == CVec3::new(PLOT_SIZE, PLOT_SIZE, PLOT_SIZE));
    assert!(CVec3::new(20, 20, 20) + ivec3(-1, -1, -1) == CVec3::new(19, 19, 19));
}

#[derive(PartialEq, PartialOrd, Debug, Eq, Hash, Clone, Copy)]
pub struct CVec3 {
    pub x: c32,
    pub y: c32,
    pub z: c32,
}

impl CVec3 {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        CVec3 {
            x: c32::new(x as i32),
            y: c32::new(y as i32),
            z: c32::new(z as i32),
        }
    }
}

impl Add for CVec3 {
    type Output = CVec3;
    fn add(self, rhs: Self) -> Self::Output {
        CVec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<IVec3> for CVec3 {
    type Output = CVec3;
    fn add(self, rhs: IVec3) -> Self::Output {
        CVec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

pub fn noise_func(t: CVec3, size: f32) -> bool {
    let vec = Vec3::new(t.x.v as f32, t.y.v as f32, t.z.v as f32);

    vec.distance(Vec3::new(
        PLOT_SIZE as f32 / 2.0,
        PLOT_SIZE as f32 / 2.0,
        PLOT_SIZE as f32 / 2.0,
    )) < size
}

pub fn add_ivec3(a: IVec3, b: IVec3) -> IVec3 {
    IVec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}
