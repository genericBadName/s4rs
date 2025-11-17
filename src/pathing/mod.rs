pub mod data;
pub mod algorithm;
pub mod math;
pub mod action;
pub mod world;

// TODO: honestly, replace this with a const fn.
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

/// Creates a `Vector2i` without needing to invoke the constructor directly.
#[macro_export]
macro_rules! vec2i {
    ($x:expr, $y:expr) => {
        Vector2i {
            x: $x,
            y: $y
        }
    };
}