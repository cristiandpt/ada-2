mod models;
mod modex;

use models::agent::Agent;
use models::social_network::SocialNetwork;
use modex::modex::individual_effort;

use std::fs;
use std::env;

use std::fs::File;
use std::io::{BufReader, BufRead};

extern crate ndarray;



fn main() {

    let mut social_network = SocialNetwork {
        resouces: 35,
        agents: vec![]
    };

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
                                                            social_network.agents.push(Agent { opinion, receptivity });
                                                        },
                                                        Err(error) => {
                                                            println!("{}", error);
                                                            return
                                                        }
                                                    }
                                                }
                                                Err(error) => {
                                                    println!("{}", error);
                                                    return
                                                } 
                                            }                 
                                    }
                                  }
                                }
                            },
                            Err(error) => {
                                println!("{}", error);
                                return
                            }  
                        }
                        
                    }
                }
                Err(error) => {
                    println!("{}", error);
                    return
                }
            }
        },
        Err(error) => {
            println!("{}", error);
            return
       },
    }

    
   
/* 
    for (index, value) in social_network.agents.iter().enumerate() {
        match w {
            0 => 0,
            x if x > 0 => i32::MIN,
            _ => std::cmp::max(me.opinion + std::cmp::max(0, w), std::cmp::max(me.opinion, 0)),
        }
    } */

    

    println!("Social_network: {:?}", social_network);
}


/* fn meme(me: Agent, w: i32) -> i32 {
 match w {
     0 => 0,
     x if x > 0 => i32::MIN,
     _ => std::cmp::max(me.opinion + std::cmp::max(0, w), std::cmp::max(me.opinion, 0)),
 }
} */


/* fn bottom_up(  ) {
    let n = social_network.agents.len();
    let m = social_network.resouces;

    let memorized_matrix: Array<f64> = Array2::from_elem((n, m), f64::INFINITY);

    for (i, agent) in social_network.agents.iter().enumerate() {
        for remain_w in (0..social_network.resouces).rev() {
            let mut agent_individual_effort = individual_effort(agent).ceil();
            memorized_matrix[i][remain_w] = match agent_individual_effort {
                x if x > remain_w => 0
            }
        }
    }
} */