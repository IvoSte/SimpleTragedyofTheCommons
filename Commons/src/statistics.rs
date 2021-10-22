use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use csv::Writer;
use rayon::prelude::*;
use serde::Serialize;

use crate::agent::structs::QTable;
use crate::config::ExperimentConfig;

pub trait Statistics {
    fn report(&self);
}

pub struct EpochStatistics {
    epoch_number: i32,
    pub alive_agents: i32,
    resources_in_pool: i32,
    chosen_actions: HashMap<i32, i32>,
}

impl EpochStatistics {
    pub fn new(
        epoch_number: i32,
        alive_agents: i32,
        resources_in_pool: i32,
        chosen_actions: HashMap<i32, i32>,
    ) -> EpochStatistics {
        EpochStatistics {
            epoch_number,
            alive_agents,
            resources_in_pool,
            chosen_actions,
        }
    }
}

impl Statistics for EpochStatistics {
    fn report(&self) {
        println!(
            "Epoch #{:<4} | agents alive: {:>3} | pool size: {:>5}",
            self.epoch_number, self.alive_agents, self.resources_in_pool
        )
    }
}

#[derive(Serialize)]
struct GenerationCsvRecord {
    gen_num: i32,
    epochs_ran: i32,
    reached_equilibrium: bool,
    agents_alive: i32,
}

pub struct GenerationStatistics {
    generation_number: i32,
    epochs_stats: Vec<EpochStatistics>,
    pub reached_equilibrium: bool,
    pub agents_alive: i32,
}

impl GenerationStatistics {
    pub fn new(
        generation_number: i32,
        epochs_stats: Vec<EpochStatistics>,
        reached_equilibrium: bool,
        agents_alive: i32,
    ) -> GenerationStatistics {
        GenerationStatistics {
            generation_number,
            epochs_stats,
            reached_equilibrium,
            agents_alive,
        }
    }

    fn as_csv_record(&self) -> GenerationCsvRecord {
        GenerationCsvRecord {
            gen_num: self.generation_number,
            epochs_ran: self.epochs_stats.len() as i32,
            reached_equilibrium: self.reached_equilibrium,
            agents_alive: self.agents_alive,
        }
    }
}

impl Statistics for GenerationStatistics {
    fn report(&self) {
        println!(
            "Generation #{:<3} | terminated after #{:<4} epochs | reached equilibrium: {}",
            self.generation_number,
            self.epochs_stats.len(),
            if self.reached_equilibrium {
                "yes"
            } else {
                "no"
            }
        )
    }
}

pub struct ExperimentStatistics {
    generations_stats: Vec<GenerationStatistics>,
    rl_stats: RLStatistics,
}

impl ExperimentStatistics {
    pub fn new(
        generations_stats: Vec<GenerationStatistics>,
        rl_stats: RLStatistics,
    ) -> ExperimentStatistics {
        ExperimentStatistics {
            generations_stats,
            rl_stats,
        }
    }

    pub fn to_csvs(&self, output_dir: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
        let mut path_1 = output_dir.clone();
        path_1.push("gen_stats.csv");
        let mut path_2 = output_dir.clone();
        path_2.push("rl_stats.csv");
        println!(
            "Writing generation stats to {}, reinforcement learning stats to {}",
            path_1.display(),
            path_2.display()
        );
        self.gen_stats_to_csv(&path_1)?;
        self.rl_stats_to_csv(&path_2)?;
        Ok(())
    }

    pub fn gen_stats_to_csv(&self, output_dir: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
        let mut out_writer = Writer::from_path(output_dir)?;
        for gen in &self.generations_stats {
            out_writer.serialize(gen.as_csv_record())?;
        }

        out_writer.flush()?;
        Ok(())
    }

    pub fn rl_stats_to_csv(&self, output_dir: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
        self.rl_stats.to_csv(output_dir)
    }
}

impl Statistics for ExperimentStatistics {
    fn report(&self) {
        println!(
            "Experiment completed | generations ran: {}",
            self.generations_stats.len()
        )
    }
}

pub struct RLStatistics {
    qtable: QTable,
}

impl RLStatistics {
    pub fn new(qtable: QTable) -> RLStatistics {
        RLStatistics { qtable }
    }

    fn csv_head(&self) -> Vec<String> {
        let mut head: Vec<String> = Vec::new();
        head.push("action_num".to_string());
        for key in self.qtable.state_action_pairs.keys() {
            head.push(key.clone());
        }
        head
    }

    fn as_csv_record(&self, action_num: usize) -> Vec<f32> {
        let mut action_evs: Vec<f32> = Vec::new();
        action_evs.push(action_num as f32);
        for (_key, value) in &self.qtable.state_action_pairs {
            action_evs.push(value[action_num].get_expected_value());
        }
        action_evs
    }

    pub fn to_csv(&self, output_dir: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
        let config: ExperimentConfig = Default::default();

        let mut out_writer = Writer::from_path(output_dir)?;
        out_writer.serialize(self.csv_head())?;
        for action_idx in 0..config.n_actions as usize {
            out_writer.serialize(self.as_csv_record(action_idx))?;
        }
        out_writer.flush()?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct AverageGenerationStatistics {
    generation_number: i32,
    avg_epochs_ran: f32,
    avg_agents_alive: f32,
}

pub struct AverageExperimentStatistics {
    avg_gen_stats: Vec<AverageGenerationStatistics>,
}

impl AverageExperimentStatistics {
    pub fn from_vector(
        experiments_stats: &Vec<ExperimentStatistics>,
    ) -> AverageExperimentStatistics {
        let n_gens = experiments_stats[0].generations_stats.len();
        let n_experiments = experiments_stats.len();
        if experiments_stats
            .iter()
            .any(|stats| stats.generations_stats.len() != n_gens)
        {
            panic!("Not all experiments ran for the same number of generations.");
        }

        let avg_gen_stats: Vec<AverageGenerationStatistics> = (0..n_gens)
            .into_par_iter()
            .map(|gen_idx| {
                let mut sum_epochs_ran: f32 = 0.;
                let mut sum_agents_alive: f32 = 0.;
                for stats in experiments_stats {
                    let gen_stats = &stats.generations_stats[gen_idx];
                    sum_epochs_ran += gen_stats.epochs_stats.len() as f32;
                    sum_agents_alive += gen_stats.agents_alive as f32;
                }

                AverageGenerationStatistics {
                    generation_number: gen_idx as i32,
                    avg_epochs_ran: sum_epochs_ran / n_experiments as f32,
                    avg_agents_alive: sum_agents_alive / n_experiments as f32,
                }
            })
            .collect();

        AverageExperimentStatistics { avg_gen_stats }
    }

    pub fn to_csv(&self, output_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut out_writer = Writer::from_path(output_dir)?;
        for gen in &self.avg_gen_stats {
            out_writer.serialize(gen)?;
        }

        out_writer.flush()?;
        Ok(())
    }
}
