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
}

impl Default for ExperimentConfig {
    fn default() -> Self {
        Self {
            n_generations: 100,
            epochs_per_gen: 200,
            n_agents: 5,
            n_actions: 4,
            init_pool_size: 10,
            max_pool_size: 60,
            regrowth_rate: 1.2,
        }
    }
}
