use std::ops::{Add, Sub};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Vector3i {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Vector3i {
    pub fn distance_squared(&self, other: &Vector3i) -> f64 {
        let xd = (self.x - other.x).pow(2);
        let yd = (self.y - other.y).pow(2);
        let zd = (self.z - other.z).pow(2);

        ((xd + yd + zd) as f64).sqrt()
    }

    pub fn new(x: i32, y: i32, z: i32) -> Vector3i {
        Vector3i { x, y, z }
    }

    pub fn zero() -> Vector3i {
        Vector3i {
            x: 0,
            y: 0,
            z: 0
        }
    }
}

impl Add for Vector3i {
    type Output = Vector3i;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vector3i {
    type Output = Vector3i;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

#[macro_export]
macro_rules! vec3i {
    ($x:expr, $y:expr, $z:expr) => {
        Vector3i {
            x: $x,
            y: $y,
            z: $z
        }
    };
}