use crate::models::social_network::SocialNetwork;
use crate::models::agent::Agent;

pub fn extremism(social_network: SocialNetwork) -> f64 {
    (social_network.agents.iter().map(|agent| agent.opinion.pow(2) as f64).sum::<f64>()).sqrt() / social_network.agents.len() as f64
}

pub fn effort(social_network: SocialNetwork, strategy: i128) -> i64 {
    social_network.agents.iter().map(|agent| individual_effort(agent) as i64).sum()
}

pub fn individual_effort(agent: &Agent) -> f32 {
   ((agent.opinion.abs() as f32) * (1.0 - agent.receptivity)).ceil()
}
pub fn moderate(agent: Agent, strategy: bool) -> i32 {
    if strategy {
        0
    } else {
        agent.opinion
    }
}