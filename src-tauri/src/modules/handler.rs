use super::config_handler::{get_config, set_config, Config};
use super::socket_handler::respond_and_update_authentication_status;
use serde::Serialize;
use std::env;
use std::fs;
pub enum HandlerResult {
    DirectoryResult(Vec<(String, bool)>),
    StringResult(String),
    BooleanResult(bool),
    SharedResult(Config),
}

impl Serialize for HandlerResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            HandlerResult::DirectoryResult(vec) => vec.serialize(serializer),
            HandlerResult::StringResult(string) => string.serialize(serializer),
            HandlerResult::BooleanResult(boolean) => boolean.serialize(serializer),
            HandlerResult::SharedResult(config) => config.serialize(serializer),
        }
    }
}

#[tauri::command]
pub fn generic_handler(mut args: Vec<String>) -> Box<HandlerResult> {
    let requested_handler = args[0].clone();
    args.remove(0);
    match requested_handler.as_str() {
        "list_directory" => list_directory(args[0].clone()),
        "get_home_directory" => get_home_directory(),
        "is_path_valid" => is_path_valid(args[0].clone()),
        "get_config" => {
            let response = get_config();
            if let Ok(result) = response {
                Box::new(HandlerResult::SharedResult(result))
            } else {
                let e = response.err().unwrap();
                Box::new(HandlerResult::StringResult(e.to_string()))
            }
        }
        "set_config" => {
            let parse_result = serde_json::from_str(&args[0]);
            if parse_result.is_err() {
                return Box::new(HandlerResult::StringResult(
                    parse_result.err().unwrap().to_string(),
                ));
            }
            let config: Config = parse_result.unwrap();
            let response = set_config(config);
            if response.is_err() {
                return Box::new(HandlerResult::StringResult(
                    response.err().unwrap().to_string(),
                ));
            }
            Box::new(HandlerResult::BooleanResult(true))
        }
        "update_status" => Box::new(HandlerResult::BooleanResult(
            respond_and_update_authentication_status(args[0].parse().unwrap(), args[1].clone()),
        )),
        _ => todo!(),
    }
}

fn list_directory(args: String) -> Box<HandlerResult> {
    println!("Reading {}", args);
    let path = fs::metadata(args.clone());
    if let Err(_) = path {
        return Box::new(HandlerResult::DirectoryResult(vec![]));
    }
    let paths = fs::read_dir(args.clone()).unwrap();
    let mut result: Vec<(String, bool)> = [].to_vec();
    for path in paths {
        let metadata = fs::metadata(path.as_ref().unwrap().path().display().to_string());
        if let Err(_) = metadata {
            println!(
                "Failed: {}",
                path.as_ref().unwrap().path().display().to_string()
            );
        } else {
            result.push((
                path.unwrap().path().display().to_string(),
                metadata.unwrap().is_dir(),
            ));
        }
    }
    Box::new(HandlerResult::DirectoryResult(result))
}

fn get_home_directory() -> Box<HandlerResult> {
    println!("HOME: {}", env::var("HOME").unwrap().to_string());
    Box::new(HandlerResult::StringResult(
        env::var("HOME").unwrap().to_string(),
    ))
}

fn is_path_valid(path: String) -> Box<HandlerResult> {
    println!("Reading {}", path);
    let path = fs::metadata(path.clone());
    if let Err(_) = path {
        return Box::new(HandlerResult::BooleanResult(false));
    } else if !path.unwrap().is_dir() {
        return Box::new(HandlerResult::BooleanResult(false));
    }
    Box::new(HandlerResult::BooleanResult(true))
}
