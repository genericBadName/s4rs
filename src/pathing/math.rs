use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Vector3i {
    x: i32,
    y: i32,
    z: i32
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

    pub fn offset(a: &Vector3i, b: &Vector3i) -> Vector3i {
        Vector3i {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z
        }
    }
}

