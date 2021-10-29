// Modules
mod agent;
mod commons;
mod config;
mod experiment;
mod statistics;

// Aliases
use agent::Agent;
use commons::Commons;
use config::{CommandLineArgs, ExperimentConfig, RLParameters, SimulationConfig, StateThresholds};
use csv::Writer;
use experiment::Experiment;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use statistics::{
    AverageExperimentStatistics, ExperimentOutput, ExperimentStatistics, RLStatistics,
};
use std::fs;
use std::fs::OpenOptions;
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

/// Run experiments without any output, basically pointless but still exists for reference
fn _run_experiments(n_experiments: i32, cfg: ExperimentConfig) {
    let multi_progress = MultiProgress::new();
    for _ in 0..n_experiments {
        let pb = multi_progress.add(ProgressBar::new(cfg.n_generations as u64));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.green} {pos:>7}/{len:7} {msg}"),
        );
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
            experiment.run(pb);
        });
    }

    multi_progress.join().expect("Progress bars failed");
}

fn run_experiments_incremental_output(
    n_experiments: i32,
    cfg: ExperimentConfig,
    output_dir: PathBuf,
) {
    fs::create_dir_all(&output_dir).expect("Could not create output dir");

    let mut exp_config_path = output_dir.clone();
    exp_config_path.push("experiment.toml");
    confy::store_path(exp_config_path, cfg).expect("Could not write experiment config");

    let mut rl_params_path = output_dir.clone();
    rl_params_path.push("rl_params.toml");
    let rl_params: RLParameters = Default::default();
    confy::store_path(rl_params_path, rl_params).expect("Could not write RL parameters");

    let mut state_thresholds_path = output_dir.clone();
    state_thresholds_path.push("state_thresholds.toml");
    let state_thresholds: StateThresholds = Default::default();
    confy::store_path(state_thresholds_path, state_thresholds)
        .expect("Could not write state thresholds");

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
        fs::create_dir_all(&exp_output_dir).expect("Could not create experiment output dir");
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
            let mut gen_stats_csv_writer = Writer::from_writer(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(gen_stats_path)
                    .unwrap(),
            );
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
    multi_progress.join().expect("Progress bars failed");
    let mut avg_rl_stats_path = output_dir.clone();
    avg_rl_stats_path.push("avg_rl_stats.csv");
    match RLStatistics::average_from_vector(receiver.iter().collect()).to_csv(&avg_rl_stats_path) {
        Ok(_) => println!("Succesfully wrote average RL Statistics"),
        Err(e) => println!("Failed to write average RL Statistics: \n {}", e),
    }
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
    let cfg: ExperimentConfig = match args.config_path {
        Some(path) => confy::load_path(path).unwrap(),
        _ => Default::default(),
    };

    // TODO: Optionally, allow config file for sim
    // config, instead of command line argument
    let mut sim_config: SimulationConfig = Default::default();
    sim_config.n_experiments = args.n_experiments;

    println!(
        "Running {} experiment{} with {} generations",
        sim_config.n_experiments,
        if sim_config.n_experiments > 1 {
            "s"
        } else {
            ""
        },
        cfg.n_generations
    );

    run_experiments_incremental_output(sim_config.n_experiments, cfg, args.output_dir);
}
