pub trait Statistics {
    fn report(&self);
}

pub struct EpochStatistics {
    epoch_number: i32,
    alive_agents: i32,
    resources_in_pool: i32,
}

impl EpochStatistics {
    pub fn new(epoch_number: i32, alive_agents: i32, resources_in_pool: i32) -> EpochStatistics {
        EpochStatistics {
            epoch_number,
            alive_agents,
            resources_in_pool,
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

pub struct GenerationStatistics {
    generation_number: i32,
    terminated_at_epoch: i32,
    reached_equilibrium: bool,
}

impl GenerationStatistics {
    pub fn new(
        generation_number: i32,
        terminated_at_epoch: i32,
        reached_equilibrium: bool,
    ) -> GenerationStatistics {
        GenerationStatistics {
            generation_number,
            terminated_at_epoch,
            reached_equilibrium,
        }
    }
}

impl Statistics for GenerationStatistics {
    fn report(&self) {
        println!(
            "Generation #{:<3} | terminated at epoch #{:<4} | reached equilibrium: {}",
            self.generation_number,
            self.terminated_at_epoch,
            if self.reached_equilibrium {
                "yes"
            } else {
                "no"
            }
        )
    }
}

pub struct ExperimentStatistics {}
