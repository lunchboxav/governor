extern crate serde_yaml;

use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;

use std::io;
use std::io::prelude::*;

use serde_json::{Value, Map};

fn main() -> Result<(), Box<Error>> {
    // // Read YAML and store it as variable.
    // let f = File::open("test-token.yaml")?;
    // let data: serde_yaml::Value = serde_yaml::from_reader(f)?;
    // println!("{:?}", data);

    // // Write variable to JSON.
    // let file = OpenOptions::new()
    //     .create(true)
    //     .write(true)
    //     .open("test-token.json")?;
    // serde_json::to_writer_pretty(&file, &data)?;

    // read from JSON and transform to SCSS
    let mut scss_buffer = File::create("test-token.scss")?;

    let j = File::open("test-token.json")?;
    let parsed: Value = serde_json::from_reader(j)?;
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    for (key, value) in obj {
        let vobj = value.as_object().unwrap();
        for (k, v) in vobj {
            if v.is_string() {
                let mut token = String::from("$");
                let token_val = format!("{}-{} : {};\n", key, k, v.as_str().unwrap());
                token.push_str(&token_val);
                scss_buffer.write(&token.as_bytes())?;
                println!("{}", token);
            } else {
                let v_obj = v.as_object().unwrap();
                for (k_, v_) in v_obj {
                    let mut token = String::from("$");
                    let token_val = format!("{}-{}-{} : {};\n",key, k, k_, v_.as_str().unwrap());
                    token.push_str(&token_val);
                    scss_buffer.write(&token.as_bytes())?;
                    println!("{}", token);
                }
            }
        }
    }
    Ok(())
}
