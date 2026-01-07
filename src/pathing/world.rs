use crate::config::Configuration;
use crate::pathing::math::{Vector2i, Vector3i};
use std::rc::Rc;

/// Stores data about a space from which nodes are sampled. A `Space` can be of N-dimensions.
pub trait Space<P> where P: Copy {
    /// Returns the cost to move to this type of material in the world.
    fn material_cost(&self, pos: P) -> f64;
}

pub struct VoxelSpace {

}

impl VoxelSpace {
    pub fn new() -> VoxelSpace {
        VoxelSpace {}
    }
}

impl Space<Vector3i> for VoxelSpace {
    fn material_cost(&self, pos: Vector3i) -> f64 {
        todo!()
    }
}

/// A simple, two-dimensional space used primarily for testing. Uses a character matrix to define
/// basic structures. \
/// `O`: Start \
/// `G`: End \
/// `X`: Solid \
/// `_`: Empty \
/// `*`: Hazardous
pub struct FlatSpace {
    plane: Vec<&'static str>,
    config: Configuration
}

impl FlatSpace {
    pub fn new(plane: Vec<&'static str>, config: Configuration) -> FlatSpace {
        FlatSpace {
            plane,
            config
        }
    }
}

impl Space<Vector2i> for FlatSpace {
    fn material_cost(&self, pos: Vector2i) -> f64 {
        if pos.x < 0 || pos.y < 0  {
            return self.config.cost_inf;
        }

        if let Some(row) = self.plane.get(pos.y as usize) &&
            let Some(char_at) = row.chars().nth(pos.x as usize) {
            return match char_at {
                'O' => 1.0,
                'G' => 1.0,
                'X' => self.config.cost_inf,
                '_' => 1.0,
                '*' => 5.0,
                _ => self.config.cost_inf,
            }
        }

        self.config.cost_inf
    }
}