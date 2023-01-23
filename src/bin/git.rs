use git2;
use std::{env, process::exit};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let pos_args = env::args()
        .filter(|a| !a.starts_with("-"))
        .collect::<Vec<String>>();
    let command = pos_args.get(1).expect("Not enought args");

    let quiet = env::args().any(|a| a == "--quiet" || a == "-q");

    match command.as_str() {
        "clone" => {
            let url = pos_args.get(2).expect("Specify URL").as_str();

            let url_split = url.trim_end_matches("/").split("/");
            let dest = pos_args
                .get(3)
                .map(String::as_str)
                .or(url_split.last())
                .expect("Unable to detect destination");

            if !quiet {
                println!("URL: {url}");
                println!("Destination: {dest}");
            }

            git2::Repository::clone(url, dest).expect("Failed to clone repository");
        }
        "version" => println!("git version {VERSION}"),
        "rev-parse" => {
            let repo = git2::Repository::open_from_env().expect("Unable to open repository");
            repo.revparse(pos_args.get(2).expect("Spec required"))
                .expect("Unable to rev-parse");
        }
        cmd => {
            println!("Unexcepted command: {cmd}");
            exit(1)
        }
    }
}
