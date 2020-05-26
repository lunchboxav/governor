extern crate serde_yaml;
extern crate js_sys;

use std::fs::File;
use std::fs::OpenOptions;

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    // Read YAML and store it as variable.
    let f = File::open("token.yaml")?;
    let data: serde_yaml::Value = serde_yaml::from_reader(f)?;
    
    // Write variable to JSON.
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("token.json")?;
    serde_json::to_writer_pretty(&file, &data)?;

    Ok(())
}
