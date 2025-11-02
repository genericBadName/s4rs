use std::path::PathBuf;
use std::time::Duration;
use crate::config::Configuration;
use crate::pathing::data::HazardMultiplier;

#[test]
fn read_write() {
    let config = Configuration::new();
    let path = PathBuf::from("test_config.json");

    let res_write = config.write_config(path.as_path());
    assert!(res_write.is_ok(), "Writing config file failed!");

    let read = Configuration::read_config(path.as_path());
    assert!(read.is_ok(), "Reading config file failed!");
    assert_eq!(read.unwrap(), config, "Read data was not equal to previous data!");
}

#[test]
fn read_nonexistent() {
    let path = PathBuf::from("nonsense_path.json");
    let read = Configuration::read_config(path.as_path());
    assert!(read.is_err(), "Returned Ok() even when the file did not exist!");
}

#[test]
fn equality() {
    let config1 = Configuration {
        hazard: HazardMultiplier {
            unknown: 2,
            non_solid: 3,
            solid: 5,
            dangerous: 7,
        },
        cost_inf: 11,
        timeout: Duration::from_millis(13),
    };

    let config2 = Configuration {
        hazard: HazardMultiplier {
            unknown: 2,
            non_solid: 3,
            solid: 5,
            dangerous: 7,
        },
        cost_inf: 11,
        timeout: Duration::from_millis(13),
    };

    assert_eq!(config1, config2, "Configurations were not equal!");
}