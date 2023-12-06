use git2;
use std::{env, process::exit};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut pos_args = env::args().filter(|a| !a.starts_with("-")).skip(1);
    let command = pos_args.next().expect("Not enought args");

    let quiet = env::args().any(|a| a == "--quiet" || a == "-q");

    match command.as_str() {
        "clone" => {
            let url = pos_args.next().expect("Specify URL");
            let dest = pos_args
                .next()
                .or_else(|| {
                    url.trim_end_matches("/")
                        .split("/")
                        .last()
                        .map(str::to_string)
                })
                .expect("Unable to detect destination");

            if !quiet {
                println!("URL: {url}");
                println!("Destination: {dest}");
            }

            git2::Repository::clone(&url, dest).expect("Failed to clone repository");
        }
        "version" => println!("git version {VERSION}"),
        "rev-parse" => {
            let repo = git2::Repository::open_from_env().expect("Unable to open repository");
            repo.revparse(&pos_args.next().expect("Spec required"))
                .expect("Unable to rev-parse");
        }
        cmd => {
            println!("Unexcepted command: {cmd}");
            exit(1)
        }
    }
}
