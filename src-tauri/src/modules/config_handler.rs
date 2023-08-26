use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    shared: HashMap<String, i32>,
}

pub fn get_config() -> Result<Config, &'static str> {
    let config_path = env::var("HOME").unwrap().to_string() + "/.qshare/";
    if let Ok(config_content) = read_config_file(config_path.to_string()) {
        println!("config content: {}", config_content);
        let response = serde_json::from_str::<Config>(&config_content);
        if response.is_err() {
            println!("{}", response.err().unwrap().to_string());
            Err("Error parsing config content.")
        } else {
            return Ok(response.unwrap());
        }
    } else {
        Err("Error reading config file.")
    }
}

pub fn set_config(object: Config) -> Result<bool, String> {
    let config_path = env::var("HOME").unwrap().to_string() + "/.qshare/";
    let is_valid_path = validate_directory(config_path.clone());
    if is_valid_path.is_err() {
        return Err(is_valid_path.err().unwrap().to_string());
    }
    let file_handler = fs::File::create(config_path + "config.json");
    if file_handler.is_err() {
        return Err("Error opening config file.".to_string());
    }
    let mut file = file_handler.unwrap();
    let response = file.write_all(serde_json::to_string(&object).unwrap().as_bytes());
    if response.is_err() {
        println!("{}", response.err().unwrap().to_string());
        return Err("Error writing config file.".to_string());
    }
    Ok(true)
}

fn read_config_file(path: String) -> Result<String, String> {
    let response = validate_directory(path.clone());
    if response.is_err() {
        return Err(response.err().unwrap().to_string());
    }
    let file = fs::File::open(path + "config.json");
    if file.is_err() {
        return Err("Error opening config file.".to_string());
    }
    let mut content = String::new();
    let response = file.unwrap().read_to_string(&mut content);
    if response.is_err() {
        return Err("Error reading config file.".to_string());
    }
    Ok(content)
}

fn validate_directory(path: String) -> Result<bool, String> {
    let path_validator = fs::metadata(path.clone());
    if path_validator.is_err() {
        let response = fs::create_dir_all(path.clone());
        match response {
            Ok(_) => {
                println!("config directory created successfully");
            }
            Err(_) => return Err("Error creating config directory.".to_string()),
        }
    }
    let file_validator = fs::metadata(path.clone() + "config.json");
    if file_validator.is_err() {
        let file_handler = fs::File::create(path.clone() + "config.json");
        match file_handler {
            Ok(_) => {
                let initialization_object = serde_json::to_string(&Config {
                    shared: HashMap::new(),
                });
                if initialization_object.is_err() {
                    return Err("Error initializing config file.".to_string());
                }
                let response = file_handler
                    .unwrap()
                    .write_all(initialization_object.unwrap().as_bytes());
                match response {
                    Err(_) => return Err("Error initializing config file.".to_string()),
                    _ => {}
                }
            }
            Err(_) => return Err("Error creating config file.".to_string()),
        }
    }
    return Ok(true);
}
