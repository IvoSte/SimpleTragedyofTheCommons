// Crates

// Modules
pub mod agent;
pub mod commons;

// Aliases
use agent::Agent;
use commons::Commons;


fn make_agents(n_agents: usize) -> Vec<Agent> {
    let mut agents: Vec<Agent> = Vec::with_capacity(n_agents);

    for id in 0..n_agents {
        agents.push(Agent::new(id as u32, None));
    }

    return agents;
}

fn regrow(current_amount: u32) -> u32 {
    // TODO: pay attention that this makes sense
    return (current_amount as f32 * 1.2) as u32;
}


fn main() {
    let n_agents: usize = 10;
    let agents = make_agents(n_agents);
    let mut commons = Commons::new(100, regrow);

}
