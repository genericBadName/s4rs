use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::time::Duration;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use eyre::{eyre, Result};
use crate::pathing::data::HazardMultiplier;

/// Configuration for the pathfinding system.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Configuration {
    /// Hazard multipliers.
    pub hazard: HazardMultiplier,
    /// Effectively infinite cost. Shouldn't be infinite to prevent overflows, but big enough
    /// for nothing to realistically reach it.
    pub cost_inf: f64,
    /// Maximum allowed pathfinding time. After that, operations will return
    /// with failure states.
    pub timeout: Duration
}

impl Configuration {
    /// Creates a defaulted config.
    pub fn new() -> Configuration {
        Configuration {
            hazard: HazardMultiplier::new(),
            cost_inf: 100_000.0,
            timeout: Duration::from_millis(2000)
        }
    }
    /// Writes the configuration file to a given path. Will overwite any existing configuration.
    pub fn write_config(&self, path: &Path) -> Result<()> {
        if path.exists() {
            info!("Overwriting existing configuration at {:?}", path)
        } else {
            info!("Writing new configuration to {:?}", path)
        }

        let mut writer = BufWriter::new(File::create(path)?);
        let out_str = to_string_pretty(&self)?;
        writer.write_all(out_str.as_bytes())?;

        Ok(())
    }

    pub fn read_config(path: &Path) -> Result<Configuration>{
        if !path.exists() {
            return Err(eyre!("Path does not exist, tried reading empty config!"))
        }

        let mut reader = BufReader::new(File::open(path)?);
        let mut in_str = String::new();
        reader.read_to_string(&mut in_str)?;
        let config: Configuration = from_str(&in_str)?;

        Ok(config)
    }
}