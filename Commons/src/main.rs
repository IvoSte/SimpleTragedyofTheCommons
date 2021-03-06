// Modules
mod agent;
mod commons;
mod config;
mod experiment;
mod statistics;

// Aliases
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::sync::mpsc::channel;

use csv::WriterBuilder;
use dialoguer::Confirm;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use structopt::StructOpt;

use agent::structs::AgentState;
use agent::Agent;
use commons::Commons;
use config::{CommandLineArgs, Config};
use experiment::Experiment;
use statistics::{
    AverageExperimentStatistics, ExperimentOutput, ExperimentStatistics, GenerationStatistics,
    RLStatistics,
};

static CONFIG: Lazy<Config> = Lazy::new(|| match CommandLineArgs::from_args().config_path {
    Some(path) => confy::load_path(path).unwrap(),
    _ => Default::default(),
});

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

fn run_experiments_incremental_output(
    n_experiments: i32,
    output_dir: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let cfg = CONFIG.experiment;

    fs::create_dir_all(&output_dir)?;

    let mut exp_config_path = output_dir.clone();
    exp_config_path.push("experiment.toml");
    confy::store_path(exp_config_path, *CONFIG)?;

    let multi_progress = MultiProgress::new();
    let (sender, receiver) = channel();
    for exp_idx in 0..n_experiments {
        let pb = multi_progress.add(ProgressBar::new(cfg.n_generations as u64));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.green} {pos:>7}/{len:7} {msg}"),
        );
        let new_sender = sender.clone();
        let mut exp_output_dir = output_dir.clone();
        exp_output_dir.push(exp_idx.to_string());
        fs::create_dir_all(&exp_output_dir)?;
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
            let mut gen_stats_path = exp_output_dir.clone();
            gen_stats_path.push("gen_stats.csv");
            let mut gen_stats_csv_writer = WriterBuilder::new().has_headers(false).from_writer(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(gen_stats_path)
                    .unwrap(),
            );
            gen_stats_csv_writer
                .write_record(GenerationStatistics::csv_header())
                .expect("Could not write gen stats header");
            let rl_stats = experiment.run_incremental_output(pb, &mut gen_stats_csv_writer);
            let mut rl_stats_path = exp_output_dir.clone();
            rl_stats_path.push("rl_stats.csv");
            match rl_stats.to_csv(&rl_stats_path) {
                Ok(_) => (),
                Err(e) => println!("Failed to write average RL Statistics: \n {}", e),
            };
            new_sender.send(rl_stats).unwrap();
        });
    }

    drop(sender);
    multi_progress.join()?;
    let mut avg_rl_stats_path = output_dir.clone();
    avg_rl_stats_path.push("avg_rl_stats.csv");
    match RLStatistics::average_from_vector(receiver.iter().collect()).to_csv(&avg_rl_stats_path) {
        Ok(_) => println!("Succesfully wrote average RL Statistics"),
        Err(e) => println!("Failed to write average RL Statistics: \n {}", e),
    }

    Ok(())
}

/// Write stats from a vector of experiment statistics
/// Currently not in use because of memory limitations
fn _write_stats(stats: Vec<ExperimentStatistics>, output_dir: PathBuf) {
    // If a csv output path is given, attempt to write the experiment results to it
    // TODO: validate this path is usable before running the whole experiment
    let mut single_output_dir = output_dir.clone();
    single_output_dir.push("experiments");
    for (exp_idx, exp_stats) in stats.iter().enumerate() {
        let mut exp_output_dir = single_output_dir.clone();
        exp_output_dir.push(exp_idx.to_string());
        fs::create_dir_all(&exp_output_dir).expect("Could not create experiment output dir");
        match exp_stats.to_csvs(&exp_output_dir) {
            Ok(_) => println!(
                "Experiment #{} statistics succesfully written to {}",
                exp_idx,
                exp_output_dir.display()
            ),
            Err(e) => println!(
                "Failed to write experiment #{} statistics: \n {}",
                exp_idx, e
            ),
        };
    }

    let mut avg_output_dir = output_dir.clone();
    avg_output_dir.push("avg_stats");
    fs::create_dir_all(&avg_output_dir).expect("Could not create output dir");
    match AverageExperimentStatistics::from_vector(&stats).to_csvs(&avg_output_dir) {
        Ok(_) => println!(
            "Average statistics succesfully written to {}",
            avg_output_dir.display()
        ),
        Err(e) => println!("Failed to write average statistics: \n {}", e),
    };
}

fn main() {
    let args = CommandLineArgs::from_args();

    // TODO: Optionally, allow config file for sim
    // config, instead of command line argument

    println!(
        "Running {} experiment{} with {} generations",
        CONFIG.simulation.n_experiments,
        if CONFIG.simulation.n_experiments > 1 {
            "s"
        } else {
            ""
        },
        CONFIG.experiment.n_generations
    );

    if args.output_dir.as_path().exists() {
        if Confirm::new()
            .with_prompt("Output directory already exists. Remove old contents?")
            .interact()
            .unwrap()
        {
            match fs::remove_dir_all(&args.output_dir) {
                Ok(_) => println!("Removed {} and its contents", args.output_dir.display()),
                Err(e) => println!(
                    "Failed to remove output directory and / or its contents: \n {}",
                    e
                ),
            }
        } else {
            println!("Not removing existing output directory. Please run the simulation with a different output directory.");
            return;
        }
    }

    if let Err(e) =
        run_experiments_incremental_output(CONFIG.simulation.n_experiments, args.output_dir)
    {
        eprintln!("Error while running experiment: {}", e);
    }
}
