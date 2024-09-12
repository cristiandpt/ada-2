use super::models::social_network::SocialNetwork;
use super::models::agent::Agent;

pub fn extremism(social_network: SocialNetwork) -> i32 {
    (social_network.agents.iter().map(|agent| agent.opinion.pow(2)).sum()).sqrt() / social_network.agents.len()
}

pub fn effort(social_network: SocialNetwork, strategy: i128) -> i64 {
    social_network.agents.iter().map(|agent| individual_effort(agent)).sum()
}

pub fn individual_effort(agent: Agent) -> i64 {
   agent.opinion.abs() * (1 - agent.receptivity)
}
pub fn moderate(agent: Agent, strategy: bool) -> i64 {
    if strategy {
        0
    } else {
        agent.opinion
    }
}