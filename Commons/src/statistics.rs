use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
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

    pub fn append_to_csv(&self, writer: &mut Writer<File>) -> Result<(), csv::Error> {
        writer.serialize(self.as_csv_record())
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

pub trait ExperimentOutput {
    fn to_csvs(&self, output_dir: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
        let mut path_1 = output_dir.clone();
        path_1.push("gen_stats.csv");
        let mut path_2 = output_dir.clone();
        path_2.push("rl_stats.csv");
        self.gen_stats_to_csv(&path_1)?;
        self.rl_stats_to_csv(&path_2)?;
        Ok(())
    }

    fn gen_stats_to_csv(&self, output_path: &std::path::PathBuf) -> Result<(), Box<dyn Error>>;

    fn rl_stats_to_csv(&self, output_path: &std::path::PathBuf) -> Result<(), Box<dyn Error>>;
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
}

impl ExperimentOutput for ExperimentStatistics {
    fn gen_stats_to_csv(&self, output_path: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
        let mut out_writer = Writer::from_path(output_path)?;
        for gen in &self.generations_stats {
            out_writer.serialize(gen.as_csv_record())?;
        }

        out_writer.flush()?;
        Ok(())
    }

    fn rl_stats_to_csv(&self, output_path: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
        self.rl_stats.to_csv(output_path)
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

#[derive(Serialize)]
pub struct AverageGenerationStatistics {
    generation_number: i32,
    avg_epochs_ran: f32,
    avg_agents_alive: f32,
}

pub struct AverageExperimentStatistics {
    avg_gen_stats: Vec<AverageGenerationStatistics>,
    avg_rl_stats: RLStatistics,
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

        let q_tables: Vec<&QTable> = experiments_stats
            .iter()
            .map(|stats| stats.rl_stats.get_q_table())
            .collect();

        AverageExperimentStatistics {
            avg_gen_stats,
            avg_rl_stats: RLStatistics::new(QTable::average_from_vector(&q_tables)),
        }
    }
}

impl ExperimentOutput for AverageExperimentStatistics {
    fn gen_stats_to_csv(&self, output_path: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
        let mut out_writer = Writer::from_path(output_path)?;
        for gen in &self.avg_gen_stats {
            out_writer.serialize(gen)?;
        }

        out_writer.flush()?;
        Ok(())
    }

    fn rl_stats_to_csv(&self, output_path: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
        self.avg_rl_stats.to_csv(output_path)
    }
}

pub struct RLStatistics {
    q_table: QTable,
}

impl RLStatistics {
    pub fn new(q_table: QTable) -> RLStatistics {
        RLStatistics { q_table }
    }

    pub fn average_from_vector(rl_stat_objects: Vec<RLStatistics>) -> RLStatistics {
        RLStatistics {
            q_table: QTable::average_from_vector(
                &rl_stat_objects
                    .iter()
                    .map(|rl_stats| rl_stats.get_q_table())
                    .collect(),
            ),
        }
    }

    fn csv_head(&self) -> Vec<String> {
        let mut head: Vec<String> = Vec::new();
        head.push("action_num".to_string());
        for key in self.q_table.state_action_pairs.keys() {
            head.push(key.clone());
        }
        head
    }

    fn as_csv_record(&self, action_num: usize) -> Vec<f32> {
        let mut action_evs: Vec<f32> = Vec::new();
        action_evs.push(action_num as f32);
        for (_key, value) in &self.q_table.state_action_pairs {
            action_evs.push(value[action_num].get_expected_value());
        }
        action_evs
    }

    pub fn to_csv(&self, output_path: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
        let mut out_writer = Writer::from_path(output_path)?;
        out_writer.serialize(self.csv_head())?;
        for action_idx in 0..self.q_table.n_actions as usize {
            out_writer.serialize(self.as_csv_record(action_idx))?;
        }
        out_writer.flush()?;
        Ok(())
    }

    pub fn get_q_table(&self) -> &QTable {
        &self.q_table
    }
}
