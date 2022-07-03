use std::io;
use std::process::exit;
const GIT_HOME_DIR: &str = ".config/git_home";

mod args;
mod git_functions;
mod run;

use args::*;
use git_functions::*;
use run::*;

fn print_add_help() {
    println!("git home add <file>");
}

fn print_usage() -> io::Result<()> {
    println!("git home [command]\n");
    println!("Commands:");
    println!("\t    add: add a file to the git_home repo.");
    println!("\t status: print staus of files in the index.");
    println!("\t   init: initialize a new home repo.");

    Ok(())
}

fn main() -> io::Result<()> {
    let args = format_args();

    match args.mode {
        ProgMode::Add => run_add(args),
        ProgMode::Init => run_init(),
        ProgMode::Status => {
            let repo = open_home_repo()?;
            if let Err(e) = print_repo_status(&repo) {
                eprintln!("git error: {}", e);
                exit(74);
            }
            Ok(())
        }
        ProgMode::None => {
            print_usage()?;
            exit(64);
        }
    }
}
