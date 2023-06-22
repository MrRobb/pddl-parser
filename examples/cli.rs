use std::path::PathBuf;

use clap::Parser;
use pddl_parser::domain::domain::Domain;
use pddl_parser::plan::Plan;
use pddl_parser::problem::Problem;

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    /// Sets the level of verbosity
    #[clap(short, long)]
    pub verbose: bool,

    /// Domain file
    #[clap(short, long)]
    pub domain: Option<PathBuf>,

    /// Problem file
    #[clap(short, long)]
    pub problem: Option<PathBuf>,

    /// Plan file
    #[clap(long)]
    pub plan: Option<PathBuf>,
}

fn main() {
    // Logger
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    // Args
    let args = Args::parse();

    if let Some(domain_file) = args.domain {
        log::info!("Domain file: {:?}", domain_file);
        let domain_str = std::fs::read_to_string(domain_file).unwrap();
        let d = Domain::parse(domain_str.as_str().into());
        if let Err(e) = d {
            log::error!("Domain Error: {:?}", e);
        }
    }

    if let Some(problem_file) = args.problem {
        log::info!("Problem file: {:?}", problem_file);
        let problem_str = std::fs::read_to_string(problem_file).unwrap();
        let p = Problem::parse(problem_str.as_str().into());
        if let Err(e) = p {
            log::error!("Problem Error: {:?}", e);
        }
    }

    if let Some(plan_file) = args.plan {
        log::info!("Plan file: {:?}", plan_file);
        let plan_str = std::fs::read_to_string(plan_file).unwrap();
        let p = Plan::parse(plan_str.as_str().into());
        if let Err(e) = p {
            log::error!("Plan Error: {:?}", e);
        }
    }
}
