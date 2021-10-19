use std::collections::HashMap;
use std::error::Error;

use csv::Writer;
use serde::Serialize;

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
}

impl ExperimentStatistics {
    pub fn new(generations_stats: Vec<GenerationStatistics>) -> ExperimentStatistics {
        ExperimentStatistics { generations_stats }
    }

    pub fn to_csv(&self, out_path: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
        let mut out_writer = Writer::from_path(out_path)?;
        for gen in &self.generations_stats {
            out_writer.serialize(gen.as_csv_record())?;
        }

        out_writer.flush()?;
        Ok(())
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
