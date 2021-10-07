// Crates
// extern crate dotenv;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate float_ord;
#[macro_use]
extern crate getset;

// Modules
mod agent;
mod commons;
mod experiment;
mod statistics;

// Aliases
use agent::Agent;
use commons::Commons;
use dotenv::dotenv;
use experiment::Experiment;

fn make_agents(n_agents: usize) -> Vec<Agent> {
    let mut agents: Vec<Agent> = Vec::with_capacity(n_agents);

    for id in 0..n_agents {
        agents.push(Agent::new(id, None));
    }
    return agents;
}

fn regrow(current_amount: i32) -> i32 {
    // TODO: pay attention that this makes sense --> whole integers and floats mixed
    // Cap at certain amount (?)
    return (current_amount as f32 * 1.2) as i32;
}

fn main() {
    dotenv().ok();
    let mut experiment = Experiment::new(
        dotenv!("N_GENERATIONS").parse().unwrap(),
        dotenv!("N_EPOCHS").parse::<usize>().unwrap(),
        make_agents(dotenv!("N_AGENTS").parse::<usize>().unwrap()),
        Commons::new(dotenv!("INIT_POOL_SIZE").parse::<i32>().unwrap(), regrow),
    );
    experiment.run();
}
