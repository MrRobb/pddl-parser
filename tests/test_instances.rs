#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};

    use git2::build::{CheckoutBuilder, RepoBuilder};
    use git2::{FetchOptions, RemoteCallbacks};
    use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
    use pddl_parser::domain::Domain;
    use pddl_parser::error::ParserError;
    use url::Url;

    fn clone(repo: &Url, path: &Path) {
        let pb = RefCell::new(
            ProgressBar::new(0).with_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] {msg} [{wide_bar:.cyan/blue}] {pos}/{total} ({eta})",
                    )
                    .unwrap()
                    .progress_chars("=> "),
            ),
        );

        let mut cb = RemoteCallbacks::new();
        cb.transfer_progress(|stats| {
            let pb = pb.borrow_mut();
            pb.set_message("Fetching...");
            pb.set_length(stats.total_objects() as u64);
            pb.set_position(stats.received_objects() as u64);
            true
        });

        let mut co = CheckoutBuilder::new();
        co.progress(|path, cur, total| {
            let pb = pb.borrow_mut();
            pb.set_message(
                path.map(std::path::Path::to_string_lossy)
                    .unwrap_or_default()
                    .to_string(),
            );
            pb.set_length(total as u64);
            pb.set_position(cur as u64);
        });

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(cb);
        RepoBuilder::new()
            .fetch_options(fo)
            .with_checkout(co)
            .clone(repo.as_str(), Path::new(path))
            .unwrap();
    }

    fn get_domain_files(folder: &Path) -> Vec<PathBuf> {
        let domain_file = folder.join("domain.pddl");
        if domain_file.exists() {
            vec![domain_file]
        }
        else {
            let folder = folder.join("domains");
            folder
                .read_dir()
                .unwrap()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().unwrap().is_file())
                .map(|e| e.path())
                .filter(|p| p.extension().map_or(false, |e| e == "pddl") && p.starts_with("domain"))
                .collect()
        }
    }

    fn is_hidden(path: &Path) -> bool {
        path.file_name()
            .and_then(OsStr::to_str)
            .map_or(false, |s| s.starts_with('.'))
    }

    #[test]
    #[ignore]
    fn generate_files() {
        // Create temporary directory
        let tempdir = tempfile::tempdir().unwrap();

        // Clone the PDDL repository
        let repo = Url::parse("https://github.com/potassco/pddl-instances").unwrap();

        // Clone the repository
        // // Path is tests/pddl-instances
        // let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        //     .join("tests")
        //     .join("pddl-instances");
        clone(&repo, tempdir.path());

        // Iterate over all instances
        // path
        let files = tempdir
            .path()
            .read_dir()
            .unwrap()
            .map(|ipc_year| ipc_year.unwrap().path())
            .filter(|ipc_year_folder| ipc_year_folder.is_dir() && !is_hidden(ipc_year_folder))
            .map(|ipc_year_folder| ipc_year_folder.join("domains"))
            .flat_map(|domains_folder| {
                domains_folder
                    .read_dir()
                    .unwrap_or_else(|_| panic!("No domains folder named {domains_folder:?}"))
                    .flat_map(|domains| get_domain_files(&domains.unwrap().path()).into_iter())
            });

        let pb = ProgressBar::new(1825).with_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] {msg} [{wide_bar:.cyan/blue}] {pos}/{total} ({eta})")
                .unwrap()
                .progress_chars("=> "),
        );

        for path in files {
            pb.inc(1);
            let domain = std::fs::read_to_string(&path).unwrap();
            pb.println(format!("Parsing {path:?}..."));
            let res = Domain::parse(domain.as_str().into());
            match res {
                Ok(_) => (),
                Err(e) => match e {
                    ParserError::UnsupportedRequirement(_) => {},
                    ParserError::ExpectedIdentifier => panic!("Expected identifier"),
                    ParserError::ExpectedToken(t) => panic!("Expected token: {t:?}"),
                    ParserError::ParseError(e) => panic!("Parse error with error: {e:?}"),
                    ParserError::IncompleteInput(e) => panic!("Incomplete input with error: {e:?}"),
                },
            }
        }
    }
}
