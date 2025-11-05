use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use jni::JNIEnv;
use jni::objects::{JObject, JValueGen};
use serde::{Deserialize, Serialize};
use eyre::Result;
use crate::binding::jni::{JNICompatible};
use crate::{vec3i};

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

impl Display for Vector3i {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl <'local> JNICompatible<'local> for Vector3i {
    const CLASS: &'static str = "com/genericbadname/s4mc/math/Vector3i";

    fn to_jni(&self, env: &mut JNIEnv<'local>) -> Result<JObject<'local>> {
        let vec3i_class = env.find_class(Self::CLASS)?;
        Ok(env.new_object(vec3i_class, "(III)V", &[
            JValueGen::Int(self.x),
            JValueGen::Int(self.y),
            JValueGen::Int(self.z)
        ])?)
    }

    fn from_jni(env: &mut JNIEnv<'local>, object: JObject<'local>) -> Result<Self>
    where
        Self: Sized
    {
        let xv = env.call_method(&object, "x", "()I", &[])?.i()?;
        let yv = env.call_method(&object, "y", "()I", &[])?.i()?;
        let zv = env.call_method(&object, "z", "()I", &[])?.i()?;

        Ok(vec3i!(xv, yv, zv))
    }
}