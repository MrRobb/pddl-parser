use std::path::{Path, PathBuf};

use indicatif::{ProgressBar, ProgressStyle};
use pddl_parser::{domain::Domain, error::ParserError};

fn get_domain_files(folder: &Path) -> Vec<PathBuf> {
    let domain_file = folder.join("domain.pddl");
    if domain_file.exists() {
        vec![domain_file]
    } else {
        let folder = folder.join("domains");
        let domains = (1..)
            .map(|i| folder.join(format!("domain-{}.pddl", i)))
            .take_while(|f| f.exists())
            .collect();
        domains
    }
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .init()
        .ok();

    let server_addr = format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT);
    eprintln!("Serving demo profile data on {}", server_addr);

    let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();

    puffin::set_scopes_on(true);

    // Create temporary directory
    // let tempdir = tempfile::tempdir().unwrap();

    // Clone the PDDL repository
    // let repo = Url::parse("https://github.com/potassco/pddl-instances").unwrap();

    // Clone the repository
    // Path is tests/pddl-instances
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("pddl-instances");

    // Iterate over all instances
    let files = path
        .read_dir()
        .unwrap()
        .map(|ipc_year| ipc_year.unwrap().path())
        .filter(|ipc_year_folder| ipc_year_folder.is_dir() && !is_hidden(&ipc_year_folder))
        .map(|ipc_year_folder| ipc_year_folder.join("domains"))
        .flat_map(|domains_folder| {
            domains_folder
                .read_dir()
                .unwrap_or_else(|_| panic!("No domains folder named {:?}", domains_folder))
                .flat_map(|domains| get_domain_files(&domains.unwrap().path()).into_iter())
        });

    let pb = ProgressBar::new(1825).with_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {msg} [{wide_bar:.cyan/blue}] {pos}/{total} ({eta})")
            .unwrap()
            .progress_chars("=> "),
    );

    let mut good = 0;
    let mut bad = 0;
    for path in files {
        pb.inc(1);
        puffin::profile_scope!("main_loop");
        puffin::GlobalProfiler::lock().new_frame();
        pb.set_message(format!("(OK: {}, BAD: {}) Parsing {:?}", good, bad, path));
        let domain = std::fs::read_to_string(&path).unwrap();
        let res = Domain::parse(&domain);
        match res {
            Ok(_) => good += 1,
            Err(e) => match e {
                ParserError::UnsupportedRequirement(_) => {},
                ParserError::ParseError(_) => {
                    bad += 1;
                },
                ParserError::IncompleteInput(_) => {
                    bad += 1;
                },
            },
        }
    }
}