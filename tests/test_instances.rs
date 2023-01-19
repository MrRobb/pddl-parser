#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::path::{Path, PathBuf};

    use cached::proc_macro::cached;
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

    #[cached]
    fn generate_files() -> Vec<String> {
        // Create temporary directory
        let tempdir = tempfile::tempdir().unwrap();

        // Clone the PDDL repository
        let repo = Url::parse("https://github.com/potassco/pddl-instances").unwrap();

        // Clone the repository
        clone(&repo, tempdir.path());

        // Iterate over all instances
        tempdir
            .path()
            .read_dir()
            .unwrap()
            .map(|ipc_year| ipc_year.unwrap().path())
            .filter(|ipc_year_folder| ipc_year_folder.is_dir() && !is_hidden(&ipc_year_folder))
            .map(|ipc_year_folder| ipc_year_folder.join("domains"))
            .map(|domains_folder| {
                domains_folder
                    .read_dir()
                    .expect(&format!("No domains folder named {:?}", domains_folder))
                    .map(|domain| get_domain_files(&domain.unwrap().path()))
                    .map(|domains| {
                        domains
                            .into_iter()
                            .map(|domain| std::fs::read_to_string(&domain).unwrap())
                    })
                    .flatten()
            })
            .flatten()
            .collect()
    }

    #[test]
    #[ignore]
    fn parse_domain() {
        for domain in generate_files().into_iter().progress() {
            let res = Domain::parse(&domain);
            match res {
                Ok(_) => (),
                Err(e) => match e {
                    ParserError::UnsupportedRequirement(r) => {
                        panic!("Unsupported requirement: {r:?}");
                    },
                    ParserError::ParseError(_) => panic!("Parse error: {e}"),
                },
            }
        }
    }
}