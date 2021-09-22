// Crates

// Modules
mod agent;
mod commons;

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
    // TODO: pay attention that this makes sense --> whole integers and floats mixed
    // Cap at certain amount (?)
 
    return (current_amount as f32 * 1.2) as u32;
}

fn single_epoch(agents: &mut Vec<Agent>, commons: &mut Commons) {

    for agent in agents {
        if agent.is_alive() {
            agent.decide_action();
            let desired_resources = agent.desired_resources();
            let taken_resources = commons.take_resources(desired_resources);
            agent.give_resources(taken_resources as i32);
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
        agent.set_alive();
    }
}

fn run_experiment(generations: usize, epochs: usize, agents: &mut Vec<Agent>, commons: &mut Commons){
    for _ in 0..generations {
        single_generation(epochs, agents, commons);
    }
}

fn main() {
    let n_agents: usize = 10;
    let mut agents = make_agents(n_agents);
    let mut commons = Commons::new(100, regrow);
    run_experiment(1, 2, &mut agents, &mut commons);

}
