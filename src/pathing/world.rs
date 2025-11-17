use crate::config::Configuration;
use crate::pathing::math::{Vector2i, Vector3i};
use std::rc::Rc;

/// Stores data about a space from which nodes are sampled. A `Space` can be of N-dimensions.
pub trait Space<P> where P: Copy {
    /// Returns the hazard multiplier for the given location.
    fn get_hazard(&self, pos: P) -> f64;
    /// Checks if the pathing entity is able to move to the given location. This is done before any
    /// other pathing or hazard checks.
    fn can_move_to(&self, pos: P) -> bool;
}

pub struct VoxelSpace {

}

impl VoxelSpace {
    pub fn new() -> VoxelSpace {
        VoxelSpace {}
    }
}

impl Space<Vector3i> for VoxelSpace {
    fn get_hazard(&self, pos: Vector3i) -> f64 {
        todo!()
    }

    fn can_move_to(&self, pos: Vector3i) -> bool {
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
    fn get_hazard(&self, pos: Vector2i) -> f64 {
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

    fn can_move_to(&self, pos: Vector2i) -> bool {
        if pos.x < 0 || pos.y < 0  {
            return false;
        }

        if let Some(row) = self.plane.get(pos.y as usize) &&
            let Some(char_at) = row.chars().nth(pos.x as usize) {
            return match char_at {
                'O' | 'G' | '_' | '*' => true,
                _ => false,
            }
        }
        
        false
    }
}