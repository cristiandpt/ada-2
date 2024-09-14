mod models;
mod modex;
mod files_loader;

use models::agent::Agent;
use models::social_network::SocialNetwork;
use modex::modex::individual_effort;
use files_loader::file_loader;

use std::fs;
use std::env;

use std::fs::File;
use std::io::{BufReader, BufRead};

extern crate ndarray;



fn main() {

    let mut social_network = SocialNetwork {
        resouces: 35,
        agents: match file_loader::load_agents_from_file() {
            Ok(agents) => agents,
            Err(error) => panic!("Error reading file: {}", error) 
        }
    };

    

    
   
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