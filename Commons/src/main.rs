// Crates
// extern crate dotenv;
extern crate float_ord;
extern crate dotenv;

// Modules
mod agent;
mod commons;
mod experiment;

// Aliases
use agent::Agent;
use commons::Commons;
use experiment::Experiment;
use dotenv::dotenv;
use std::env;

fn make_agents(n_agents: usize) -> Vec<Agent> {
    let mut agents: Vec<Agent> = Vec::with_capacity(n_agents);

    for id in 0..n_agents {
        agents.push(Agent::new(id as i32, None));
    }
    return agents;
}

fn regrow(current_amount: i32) -> i32 {
    // TODO: pay attention that this makes sense --> whole integers and floats mixed
    // Cap at certain amount (?)
    return (current_amount as f32 * 1.2) as i32;
}

fn main() {
    dotenv::dotenv().ok();
    let n_agents: usize = env::var("NUMBER_OF_AGENTS").unwrap_or_default().parse::<usize>().unwrap();
    let mut agents = make_agents(n_agents);
    let mut commons = Commons::new(100, regrow);
    let experiment = Experiment::new(1, 2, &mut agents, &mut commons);
    experiment.run();
}
