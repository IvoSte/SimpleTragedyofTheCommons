// Modules
mod agent;
mod commons;
mod config;
mod experiment;
mod statistics;

// Aliases
use agent::Agent;
use commons::Commons;
use config::{CommandLineArgs, ExperimentConfig, SimulationConfig};
use experiment::Experiment;
use rayon::prelude::*;
use statistics::{AverageExperimentStatistics, ExperimentStatistics};
use structopt::StructOpt;

fn make_agents(n_agents: i32, n_actions: i32) -> Vec<Agent> {
    let mut agents: Vec<Agent> = Vec::with_capacity(n_agents as usize);

    for id in 0..n_agents {
        agents.push(Agent::new(id, None, n_actions));
    }
    return agents;
}

fn regrow(current_amount: i32, regrowth_rate: f32) -> i32 {
    (current_amount as f32 * regrowth_rate) as i32
}

fn main() {
    let args = CommandLineArgs::from_args();
    let cfg: ExperimentConfig = match args.config_path {
        Some(path) => confy::load_path(path).unwrap(),
        _ => Default::default(),
    };

    let sim_config: SimulationConfig = Default::default();

    let n_experiments = if let Some(n_exps) = args.n_experiments {
        n_exps
    } else {
        sim_config.n_experiments
    };

    let experiment_results: Vec<ExperimentStatistics> = (0..n_experiments)
        .into_par_iter()
        .map(|_| {
            let mut experiment = Experiment::new(
                cfg.n_generations,
                cfg.epochs_per_gen,
                make_agents(cfg.n_agents, cfg.n_actions),
                Commons::new(
                    cfg.init_pool_size as i32,
                    cfg.max_pool_size as i32,
                    regrow,
                    cfg.regrowth_rate,
                ),
                cfg,
            );

            experiment.run()
        })
        .collect();

    let avg_stats = AverageExperimentStatistics::from_vector(experiment_results);

    // If a csv output path is given, attempt to write the experiment results to it
    // TODO: validate this path is usable before running the whole experiment
    if let Some(out_path) = args.out_path {
        match avg_stats.to_csv(&out_path) {
            Ok(_) => println!(
                "Experiment statistics succesfully written to {}",
                out_path.display()
            ),
            Err(e) => println!("Failed to write statistics: \n {}", e),
        };
    }
}
