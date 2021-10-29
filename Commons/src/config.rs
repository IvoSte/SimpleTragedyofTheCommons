use structopt::StructOpt;

use serde::{Deserialize, Serialize};

#[derive(StructOpt)]
#[structopt(
    name = "Tragedy of the Commons",
    about = "A simulation of the Tragedy of the Commons using Q-learning agents."
)]
pub struct CommandLineArgs {
    /// Path to output csv file
    #[structopt(parse(from_os_str))]
    pub output_dir: std::path::PathBuf,

    /// Path to the experiment configuration file
    #[structopt(short, long = "config_path", parse(from_os_str))]
    pub config_path: Option<std::path::PathBuf>,

    #[structopt(short, long, default_value = "1")]
    pub n_experiments: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SimulationConfig {
    pub n_experiments: i32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self { n_experiments: 1 }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub experiment: ExperimentConfig,
    pub rl_params: RLParameters,
    pub state_thresholds: StateThresholds,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
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
            n_generations: 100000,
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

#[derive(Serialize, Deserialize)]
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
            alpha: 0.05,
            gamma: 0.9,
            death_punish: 100000,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StateThresholds {
    pub commons_low: f32,
    pub commons_med: f32,
    pub score_low: i32,
    pub score_med: i32,
}

impl Default for StateThresholds {
    fn default() -> Self {
        Self {
            commons_low: 0.3, // percentage of max commons
            commons_med: 0.7,
            score_low: 2, // times consume
            score_med: 6,
        }
    }
}
