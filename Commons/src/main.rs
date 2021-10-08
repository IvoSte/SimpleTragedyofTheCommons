// Crates
extern crate confy;
extern crate float_ord;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate serde;

// Modules
mod agent;
mod commons;
mod config;
mod experiment;
mod statistics;

// Aliases
use agent::Agent;
use commons::Commons;
use config::TragedyConfig;
use experiment::Experiment;

fn make_agents(n_agents: i32, n_actions: i32) -> Vec<Agent> {
    let mut agents: Vec<Agent> = Vec::with_capacity(n_agents as usize);

    for id in 0..n_agents {
        agents.push(Agent::new(id, None, n_actions));
    }
    return agents;
}

fn regrow(current_amount: i32, regrowth_rate: f32) -> i32 {
    // TODO: pay attention that this makes sense --> whole integers and floats mixed
    // Cap at certain amount (?)
    return (current_amount as f32 * regrowth_rate) as i32;
}

fn main() {
    //let cfg: TragedyConfig = confy::load("tragedy-of-the-commons").unwrap();
    let cfg = TragedyConfig { 
                n_generations: 100000, 
                epochs_per_gen: 2000, 
                n_agents: 1, 
                n_actions: 9, 
                init_pool_size: 5, 
                max_pool_size: 9, 
                regrowth_rate: 1.5 };
    let mut experiment = Experiment::new(
        cfg.n_generations,
        cfg.epochs_per_gen,
        make_agents(cfg.n_agents, cfg.n_actions),
        Commons::new(cfg.init_pool_size as i32, cfg.max_pool_size as i32, regrow, cfg.regrowth_rate as f32),
    );
    experiment.run();
}
