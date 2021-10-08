use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

// Aliases
use super::agent::Agent;
use super::commons::Commons;
use super::statistics::{EpochStatistics, ExperimentStatistics, GenerationStatistics, Statistics};

pub struct Experiment {
    n_generations: i32,
    epochs_per_gen: i32,
    agents: Vec<Agent>,
    commons: Commons,
}

impl Experiment {
    pub fn new(
        n_generations: i32,
        epochs_per_gen: i32,
        agents: Vec<Agent>,
        commons: Commons,
    ) -> Experiment {
        Experiment {
            n_generations,
            epochs_per_gen,
            agents,
            commons,
        }
    }

    /// Run the experiment with `self.generations` generations.
    pub fn run(&mut self) {
        for n in 0..self.n_generations {
            let result = self.single_generation(n);
            //result.report();
        }
        self.agents[0].print_score();
        self.agents[0].report_action_evs();    
    }

    /// Run one generation, executing epochs until the commons
    /// are exhausted and all agents are dead, or equilibrium.char
    fn single_generation(&mut self, generation_number: i32) -> GenerationStatistics {
        let mut reached_equilibrium = true;
        let mut current_epoch = 0;
        while current_epoch < self.epochs_per_gen {
            let results = self.single_epoch(current_epoch);
            
            //results.report();
            //self.agents[0].print_score();
            //self.agents[0].report_action_evs();    
            
            if self.agents.iter().filter(|&agent| agent.is_alive()).count() == 0 {
                reached_equilibrium = false;
                break;
            }
            current_epoch += 1;
        }
        for agent in &mut self.agents {
            agent.revive();
        }
        self.commons.reset();
        if reached_equilibrium == true {
            println!("Reached equilibrium at epoch {}", current_epoch);
        }
        GenerationStatistics::new(generation_number, current_epoch, reached_equilibrium)
    }

    /// Execute a single epoch in the generation: each agent
    /// executes one action, and the commons regrows.
    fn single_epoch(&mut self, epoch_number: i32) -> EpochStatistics {
        // Shuffle the agents vector before taking actions to avoid order-based behavior
        let mut rng = thread_rng();
        self.agents.shuffle(&mut rng);

        let mut chosen_actions: HashMap<i32, i32> = HashMap::new();

        for agent in &mut self.agents {
            if agent.is_alive() {
                agent.decide_action();
                let desired_resources = agent.desired_resources();
                *chosen_actions.entry(desired_resources).or_default() += 1;
                let taken_resources = self.commons.take_resources(desired_resources);
                agent.get_resources(taken_resources);
                agent.consume(1);
                agent.learn();
            }
        }

        // TODO: Should the commons grow at th end of the epoch, or at the start?
        self.commons.grow();

        EpochStatistics::new(
            epoch_number,
            self.agents.iter().filter(|&agent| agent.is_alive()).count() as i32,
            self.commons.resource_pool,
            chosen_actions,
        )
    }
}
