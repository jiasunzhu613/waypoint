use std::{env, fs};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::ErrorKind;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct WaypointInfo {
    pub stack: Vec<String>
}

// PathBuf is an owned value while Path is a borrowed view
fn get_waypoint_file_path() -> PathBuf {
    let mut start = env::home_dir().unwrap();
    start.push(env!("WAYPOINT_FILE"));

    start
}

fn get_waypoint_dir() -> PathBuf {
    let mut start = env::home_dir().unwrap();
    start.push(env!("WAYPOINT_DIR"));

    start
}

// todo: should this be wrapped with result?
fn read_waypoint_info() -> Result<WaypointInfo> {
    // Open file and construct it as a WaypointInfo structure
    let path = get_waypoint_file_path();
    let display = &path.display().to_string(); // gets actual file path

    // Use closure here to 
    let mut file = File::open(&path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            fs::create_dir_all(get_waypoint_dir()).unwrap();
            let mut file = File::create_new(path).unwrap_or_else(|error| {
                panic!("Problem creating file {error}");
            });
            let s = serde_json::to_string(&WaypointInfo{stack: vec!()}).unwrap();
            file.write_all(s.as_bytes()).unwrap();
            file
        } else {
            panic!("Problem opening file: {error}");
        }
    });

    file.seek(SeekFrom::Start(0)).expect("Failed to seek");

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read contents of file at {}: {}", display, why),
        Ok(_) => {} // println!("{} contains: \n{}", display, s)
    }

    // let waypoint_info: WaypointInfo = serde_json::from_str(&s)?;
    serde_json::from_str(&s)
}

// File will always exist when writing
fn write_waypoint_info(info: WaypointInfo) -> Result<()> {
    let path = get_waypoint_file_path();

    let display = path.display(); // gets actual file path
    let mut file = File::create(&path).unwrap();

    let s = serde_json::to_string(&info)?;
    match file.write_all(s.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => {} // println!("successfully wrote to: {}", display)
    }

    Ok(())
}

pub fn pushDirectory(dir: String) -> Result<()> {
    let mut waypoint_info: WaypointInfo = read_waypoint_info().unwrap(); // TODO: is this safe?

    let mut path = env::current_dir().unwrap();
    path.push(dir);

    waypoint_info.stack.push(path.to_string_lossy().into_owned()); // TODO: change this?

    write_waypoint_info(waypoint_info);

    Ok(())
}

pub fn popDirectory() -> String {
    let mut waypointInfo: WaypointInfo = read_waypoint_info().unwrap();

    let dir = match waypointInfo.stack.pop() {
        None => panic!("No directory to pop to"),
        Some(result) => result
    };
    
    write_waypoint_info(waypointInfo);

    dir
}

/// TESTS

#[cfg(test)]
mod tests {
    // Useful idiom for importing from outer scope
    use super::*;

    #[test]
    fn test_get_waypoint_file_path() {
        let expected = PathBuf::from("/Users/jonathanzhu/.waypoint/info.json");
        assert_eq!(get_waypoint_file_path(), expected);
    }

    // #[test]
    // fn test_write_waypoint_info() {
    //     let info =  WaypointInfo { stack: vec!["~".to_string()] }; 
    //     let result = write_waypoint_info(info);

    //     assert!(result.is_ok());
    // }
}