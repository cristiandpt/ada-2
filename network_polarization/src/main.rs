mod models;

use models::agent;
use models::social_network::SocialNetwok;

fn main() {
    let mut social_network = SocialNetwok {
        resouces: 35,
        agents: vec![
            agent::Agent { 
                opinion: -30, 
                receptivity: 0.9 
            },
            agent::Agent { 
                opinion: 40, 
                receptivity: 0.1 
            },
            agent::Agent { 
                opinion: 50, 
                receptivity: 0.5 
            },
            
        ]
    };
    println!("Hello, world!");
}
