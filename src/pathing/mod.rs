pub mod data;
pub mod algorithm;
pub mod math;
pub mod action;
mod world;

/// Creates a `Vector3i` without needing to invoke the constructor directly.
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