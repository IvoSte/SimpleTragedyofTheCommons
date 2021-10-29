use csv::Writer;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fs::File;

use indicatif::{ProgressBar, ProgressIterator};

use crate::agent::structs::QTable;
use crate::statistics::RLStatistics;

// Aliases
use super::agent::structs::AgentState;
use super::agent::Agent;
use super::commons::Commons;
use super::config::ExperimentConfig;
use super::statistics::{EpochStatistics, GenerationStatistics};

pub struct Experiment {
    n_generations: i32,
    epochs_per_gen: i32,
    agents: Vec<Agent>,
    commons: Commons,
    config: ExperimentConfig,
}

impl Experiment {
    pub fn new(
        n_generations: i32,
        epochs_per_gen: i32,
        agents: Vec<Agent>,
        commons: Commons,
        config: ExperimentConfig,
    ) -> Experiment {
        Experiment {
            n_generations,
            epochs_per_gen,
            agents,
            commons,
            config,
        }
    }

    pub fn run_incremental_output(
        &mut self,
        pb: ProgressBar,
        csv_writer: &mut Writer<File>,
    ) -> RLStatistics {
        for gen_idx in (0..self.n_generations).progress_with(pb) {
            let gen_stats = self.single_generation(gen_idx);
            gen_stats
                .append_to_csv(csv_writer)
                .expect("Cannot write generation stats");
        }

        RLStatistics::new(QTable::average_q_table(&self.agents))
    }

    /// Run one generation, executing epochs until the commons
    /// are exhausted and all agents are dead, or equilibrium.char
    fn single_generation(&mut self, generation_number: i32) -> GenerationStatistics {
        let mut reached_equilibrium = true;
        let mut final_agents_alive = 0;
        let mut current_epoch = 0;

        let pool = self.commons.resource_pool;
        self.agents
            .iter_mut()
            .for_each(|agent| agent.update_state(pool));

        let mut epochs_stats: Vec<EpochStatistics> =
            Vec::with_capacity(self.epochs_per_gen as usize);

        while current_epoch < self.epochs_per_gen {
            let epoch_stats = self.single_epoch(current_epoch);

            //results.report();
            //self.agents[0].print_score();
            //self.agents[0].report_action_evs();

            final_agents_alive = epoch_stats.alive_agents;

            epochs_stats.push(epoch_stats);

            if self.agents.iter().filter(|&agent| agent.is_alive()).count() == 0 {
                reached_equilibrium = false;
                break;
            }
            current_epoch += 1;
        }

        // Revive all agents and reset commons
        self.agents.iter_mut().for_each(|agent| agent.revive());
        self.commons.reset();

        GenerationStatistics::new(
            generation_number,
            epochs_stats,
            reached_equilibrium,
            final_agents_alive,
        )
    }

    /// Execute a single epoch in the generation: each agent
    /// executes one action, and the commons regrows.
    fn single_epoch(&mut self, epoch_number: i32) -> EpochStatistics {
        // Shuffle the agents vector before taking actions to avoid order-based behavior
        self.agents.shuffle(&mut thread_rng());

        let mut chosen_actions: HashMap<String, Vec<i32>> = AgentState::state_keys()
            .iter()
            .map(|state_key| {
                (
                    state_key.clone(),
                    vec![0_i32; self.config.n_actions as usize],
                )
            })
            .collect();

        for agent in &mut self.agents {
            if agent.is_alive() {
                agent.decide_action();
                let desired_resources = agent.desired_resources();
                if let Some(state) = agent.get_current_state() {
                    chosen_actions.get_mut(&state.to_string()).unwrap()
                        [desired_resources as usize] += 1;
                }
                let taken_resources = self.commons.take_resources(desired_resources);
                agent.get_resources(taken_resources);
            }
        }

        // Commons grow after agents take, and before they asses the new state.
        // In this way, they see the indirect effect of their behaviour -- they learn what
        // the new state will be, not what it is directly after.
        self.commons.grow();

        let consumption = self.config.consumption;
        let pool = self.commons.resource_pool;

        // At days end, agents consume their food, go to sleep and see what
        // their cumulative actions have done to the commons, and their own food supply
        for agent in &mut self.agents {
            if agent.is_alive() {
                // let debug_print = true if agent.get_current_state().unwrap()
                agent.consume(consumption);
                agent.update_state(pool);
                agent.learn();
            }
        }

        EpochStatistics::new(
            epoch_number,
            self.agents.iter().filter(|agent| agent.is_alive()).count() as i32,
            self.commons.resource_pool,
            chosen_actions,
        )
    }
}
