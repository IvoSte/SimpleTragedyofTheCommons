use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
pub struct CommandLineArgs {
    /// The path to the experiment configuration file
    #[structopt(parse(from_os_str))]
    pub config_path: Option<std::path::PathBuf>,
}

#[derive(Serialize, Deserialize)]
pub struct ExperimentConfig {
    pub n_generations: i32,
    pub epochs_per_gen: i32,
    pub n_agents: i32,
    pub n_actions: i32,
    pub init_pool_size: i32,
    pub max_pool_size: i32,
    pub regrowth_rate: f32,
    pub consumption: i32,
}

impl Default for ExperimentConfig {
    fn default() -> Self {
        Self {
            n_generations: 10000,
            epochs_per_gen: 100,
            n_agents: 10,
            n_actions: 5,
            init_pool_size: 120,
            max_pool_size: 120,
            regrowth_rate: 1.2,
            consumption: 1,
        }
    }
}

pub struct RLParameters {
    pub epsilon: f32,
    pub alpha: f32,
    pub gamma: f32,
    pub death_punish: i32,
}

impl Default for RLParameters {
    fn default() -> Self {
        Self {
            epsilon: 0.01,
            alpha: 0.2,
            gamma: 0.9,
            death_punish: 100000,
        }
    }
}