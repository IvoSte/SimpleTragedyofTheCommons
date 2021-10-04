// Crates
// extern crate dotenv;
extern crate float_ord;

// Modules
mod agent;
mod commons;

// Aliases
use agent::Agent;
use commons::Commons;
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

fn single_epoch(agents: &mut Vec<Agent>, commons: &mut Commons) {
    for agent in agents {
        if agent.is_alive() {
            agent.decide_action();
            let desired_resources = agent.desired_resources();
            let taken_resources = commons.take_resources(desired_resources);
            agent.get_resources(taken_resources);
            agent.consume(0);
            agent.print_score();
        }
    }

    commons.grow();
    commons.print_pool();
}

fn single_generation(epochs: usize, agents: &mut Vec<Agent>, commons: &mut Commons) {
    for _ in 0..epochs {
        single_epoch(agents, commons);
    }
    for agent in agents {
        agent.revive();
    }
}

fn run_experiment(
    generations: usize,
    epochs: usize,
    agents: &mut Vec<Agent>,
    commons: &mut Commons,
) {
    for _ in 0..generations {
        single_generation(epochs, agents, commons);
    }
}

fn main() {
    dotenv::dotenv().ok();
    let n_agents: usize = 10;
    let mut agents = make_agents(n_agents); //env::var("NUMBER_OF_AGENTS").unwrap_or_default()
    let mut commons = Commons::new(100, regrow);
    run_experiment(1, 2, &mut agents, &mut commons);
}
