pub trait Statistics {
    fn report(&self);
}

pub struct EpochStatistics {
    epoch_number: usize,
    alive_agents: usize,
    pool: usize,
}

impl EpochStatistics {
    pub fn new(epoch_number: usize, alive_agents: usize, pool: usize) -> EpochStatistics {
        EpochStatistics {
            epoch_number,
            alive_agents,
            pool,
        }
    }
}

impl Statistics for EpochStatistics {
    fn report(&self) {
        println!(
            "Epoch #{:<4} | agents alive: {:>3} | pool size: {:>5}",
            self.epoch_number, self.alive_agents, self.pool
        )
    }
}

pub struct GenerationStatistics {
    generation_number: usize,
    terminated_at_epoch: usize,
    reached_equilibrium: bool,
}

impl GenerationStatistics {
    pub fn new(
        generation_number: usize,
        terminated_at_epoch: usize,
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
