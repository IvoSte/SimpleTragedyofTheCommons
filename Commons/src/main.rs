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
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use statistics::{AverageExperimentStatistics, ExperimentStatistics};
use std::path::PathBuf;
use std::sync::mpsc::channel;
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

fn run_experiments(n_experiments: i32, cfg: ExperimentConfig) -> Vec<ExperimentStatistics> {
    let multi_progress = MultiProgress::new();
    let (sender, receiver) = channel();
    for _ in 0..n_experiments {
        let pb = multi_progress.add(ProgressBar::new(cfg.n_generations as u64));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.green} {pos:>7}/{len:7} {msg}"),
        );
        let new_sender = sender.clone();
        rayon::spawn(move || {
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
            new_sender.send(experiment.run(pb)).unwrap();
        });
    }

    drop(sender);
    multi_progress.join().expect("Progress bars failed");
    receiver.iter().collect()
}

fn write_stats(stats: Vec<ExperimentStatistics>, output_dir: PathBuf) {
    // If a csv output path is given, attempt to write the experiment results to it
    // TODO: validate this path is usable before running the whole experiment
    match stats[0].to_csvs(&output_dir) {
        Ok(_) => println!(
            "Experiment statistics succesfully written to {}",
            output_dir.display()
        ),
        Err(e) => println!("Failed to write experiment statistics: \n {}", e),
    };

    let mut avg_output_path = output_dir.clone();
    avg_output_path.push("avg_gen_stats.csv");
    match AverageExperimentStatistics::from_vector(&stats).to_csv(&avg_output_path) {
        Ok(_) => println!(
            "Average statistics succesfully written to {}",
            avg_output_path.display()
        ),
        Err(e) => println!("Failed to write average statistics: \n {}", e),
    };
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

    println!(
        "Running {} experiment{} with {} generations",
        n_experiments,
        if n_experiments > 1 { "s" } else { "" },
        cfg.n_generations
    );

    let stats = run_experiments(n_experiments, cfg);
    if let Some(output_dir) = args.output_dir {
        write_stats(stats, output_dir);
    }
}
