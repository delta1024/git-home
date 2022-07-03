use super::GIT_HOME_DIR;
use git2::{Error, Repository, StatusOptions};
use std::{env, io, path::Path, process::exit};

/// Returs the home repository
pub fn open_home_repo() -> io::Result<Repository> {
    let home_dir = match env::var("HOME") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Couldn't get value of $HOME: {}", e);
            exit(1);
        }
    };

    let git_home_dir = match env::var("GIT_HOME_DIR") {
        Ok(val) => val,
        Err(_e) => format!("{}/{}", home_dir, GIT_HOME_DIR),
    };

    let repo = match Repository::open_bare(&git_home_dir) {
        Ok(repo) => repo,
        Err(_e) => {
            println!(
                "Git home repo doesn't exist, create one now at {}?",
                git_home_dir
            );
            print!("(y/n) ");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            buffer.pop();
            if buffer == "y" {
                println!("Creating git home repo: {}.", git_home_dir);
                match Repository::init_bare(&git_home_dir) {
                    Ok(repo) => repo,
                    Err(e) => {
                        eprintln!(
                            "Could not initialize git_home directory at {}: {}",
                            git_home_dir, e
                        );
                        exit(1);
                    }
                }
            } else {
                println!("You can create a new repository at any time by running 'git home init'.");
                exit(1);
            }

            // }
        }
    };

    let path = Path::new(&home_dir);
    if let Err(e) = repo.set_workdir(&path, false) {
        eprintln!("Could not set working dir to {}", e);
        exit(1);
    };
    Ok(repo)
}

/// Prints the satus of the home repo.
pub fn print_repo_status(repo: &Repository) -> Result<(), Error> {
    let mut options = StatusOptions::new();
    options.include_untracked(false);
    options.show(git2::StatusShow::IndexAndWorkdir);
    let status = repo.statuses(Some(&mut options))?;
    for i in status.iter() {
        println!(
            "{}",
            match i.path() {
                Some(path) => path,
                None => {
                    eprintln!("Path is not valid utf-8");
                    exit(1);
                }
            }
        )
    }
    Ok(())
}
