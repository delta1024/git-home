// git-home -- A dotfiles manager using git.
// Copyright (C) 2022 Jacob Stannix

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use super::GIT_HOME_DIR;
use git2::{Error, Object, Repository, Signature, StatusOptions, Tree};
use std::result;
use std::{env, io, io::prelude::*, path::Path, process::exit};

/// Gets the absolute path of the git_home_directory.
///
/// Returns `Ok(path)` if GIT_HOME_DIR env variable is set else it returns `Err(path)` with the default value.
pub fn resolve_git_repo() -> result::Result<String, String> {
    if let Ok(dir) = std::env::var("GIT_HOME_DIR") {
        Ok(dir)
    } else {
        let mut home_dir = if let Ok(home) = std::env::var("HOME") {
            home
        } else {
            String::new()
        };
        if home_dir.len() != 0 {
            home_dir.push('/');
        }
        Err(format!("{}{}", home_dir, GIT_HOME_DIR))
    }
}

/// Returs the home repository
pub fn open_home_repo() -> io::Result<Repository> {
    let git_home_dir = match resolve_git_repo() {
        Ok(string) | Err(string) => string,
    };

    let home_dir = match env::var("HOME") {
        Ok(string) => string,
        Err(_) => {
            eprintln!("Could not get value of $HOME.");
            exit(74);
        }
    };

    let repo = match Repository::open_bare(&git_home_dir) {
        Ok(repo) => repo,
        Err(_e) => {
            println!(
                "Git home repo doesn't exist, create one now at {}?",
                git_home_dir
            );
            print!("(y/n) ");
            io::stdout().flush()?;
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

/// Returns required arguments for an initial commit
pub fn gen_init_comimt_args(repo: &Repository) -> io::Result<(Signature<'static>, Tree)> {
    let sig = match repo.signature() {
        Ok(sig) => sig,
        Err(_e) => {
            eprintln!(
                "Unable to create a commit signiture.\n\
		 Perhaps 'user.name' and 'user.email' are not set"
            );
            exit(64);
        }
    };
    let mut index = match repo.index() {
        Ok(index) => index,
        Err(_e) => {
            eprintln!("Could not open repository index.");
            exit(64);
        }
    };
    let tree_id = match index.write_tree() {
        Ok(id) => id,
        Err(_e) => {
            eprintln!("Unable to write initial tree form index.");
            exit(74);
        }
    };
    let tree = match repo.find_tree(tree_id) {
        Ok(id) => id,
        Err(_e) => {
            eprintln!("Could not look up initial tree");
            exit(74);
        }
    };
    Ok((sig, tree))
}

pub fn gen_commit_args(repo: &Repository) -> io::Result<(Object, Signature<'static>, Tree)> {
    let (parent, _refrence) = match repo.revparse_ext("HEAD") {
        Ok(result) => result,
        Err(_e) => unreachable!(),
    };

    let (sig, tree) = gen_init_comimt_args(repo)?;

    Ok((parent, sig, tree))
}
