use crate::models::agent::Agent;
use super::error::MyError;

use std::fmt::Error;
use std::fs;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn load_agents_from_file() -> Result<Vec<Agent>, MyError> {
    let mut agent_network = Vec::new();
    let working_directory = env::current_dir();
    match working_directory {
        Ok(path) => {
            let resources_dir = path.join("src/resources");
            let file_path = resources_dir.join("Prueba1.txt");
            print!("{}", file_path.display());
            let file = File::open(file_path);
            match file {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    for line in reader.lines() {
                        match line {
                            Ok(line) => {
                                let words_in_line = line.split(",").collect::<Vec<&str>>();
                                if let Some(fs_opinion) = words_in_line.get(0) {
                                    if let Some(fs_receptivity) = words_in_line.get(1) { { 
                                            match fs_opinion.parse::<i32>() {
                                                Ok(opinion) => {
                                                    match fs_receptivity.parse::<f32>() {
                                                        Ok(receptivity) => {
                                                            agent_network.push(Agent { opinion, receptivity });
                                                        },
                                                        Err(error) => {
                                                            println!("{}", error);
                                                            return Err(MyError("Error reading file".to_string()))
                                                        }
                                                    }
                                                }
                                                Err(error) => {
                                                    println!("{}", error);
                                                    return Err(MyError("Error reading file".to_string()))
                                                } 
                                            }                 
                                    }
                                  }
                                }
                            },
                            Err(error) => {
                                println!("{}", error);
                                return Err(MyError("Error reading file".to_string()))
                            }  
                        }
                        
                    }
                    return Ok(agent_network)
                }
                Err(error) => {
                    println!("{}", error);
                    return Err(MyError("Error reading file".to_string()))
                }
            }
        },
        Err(error) => {
            println!("{}", error);
            return Err(MyError("Error reading file".to_string()))
       },
    }
}