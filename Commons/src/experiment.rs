use csv::Writer;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fs::File;

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

use crate::agent::structs::QTable;
use crate::statistics::RLStatistics;

// Aliases
use super::agent::Agent;
use super::commons::Commons;
use super::config::ExperimentConfig;
use super::statistics::{EpochStatistics, ExperimentStatistics, GenerationStatistics};

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

    /// Run the experiment with `self.generations` generations.
    pub fn run(&mut self, pb: ProgressBar) -> ExperimentStatistics {
        let mut reached_equilibrium = 0;
        let mut avg_agents_alive = 0;
        let mut generations_stats: Vec<GenerationStatistics> =
            Vec::with_capacity(self.n_generations as usize);
        for n in (0..self.n_generations).progress_with(pb) {
            let gen_stats = self.single_generation(n);
            if gen_stats.reached_equilibrium {
                reached_equilibrium += 1;
                avg_agents_alive += gen_stats.agents_alive;
            }
            //result.report();
            generations_stats.push(gen_stats);
        }
        // println!(
        //     "Reached equilibrium {}/{} : {}%",
        //     reached_equilibrium,
        //     self.n_generations,
        //     (reached_equilibrium as f32 / self.n_generations as f32 * 100.0)
        // );
        // println!(
        //     "average agents alive at equilibrium: {}",
        //     avg_agents_alive as f32 / reached_equilibrium as f32
        // );
        // self.agents[0].print_score();
        // self.agents[0].report_action_evs();
        let rl_stats = RLStatistics::new(QTable::average_q_table(&self.agents));
        ExperimentStatistics::new(generations_stats, rl_stats)
    }

    pub fn run_incremental_output(
        &mut self,
        pb: ProgressBar,
        csv_writer: &mut Writer<File>,
    ) -> RLStatistics {
        let mut reached_equilibrium = 0;
        let mut avg_agents_alive = 0;
        for gen_idx in (0..self.n_generations).progress_with(pb) {
            let gen_stats = self.single_generation(gen_idx);
            if gen_stats.reached_equilibrium {
                reached_equilibrium += 1;
                avg_agents_alive += gen_stats.agents_alive;
            }
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

        let mut chosen_actions: Vec<i32> = vec![0; self.config.n_actions as usize];

        for agent in &mut self.agents {
            if agent.is_alive() {
                agent.decide_action();
                let desired_resources = agent.desired_resources();
                chosen_actions[desired_resources as usize] += 1;
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

        for agent in &mut self.agents {
            if agent.is_alive() {
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
