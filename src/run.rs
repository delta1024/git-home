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

use super::{args::*, git::*, print_commit_usage};
use chrono::{Local, TimeZone};
use git2::{Repository, StatusOptions};
use std::{io, path::Path, process::exit};
/// Runs the program in add mode.
pub fn run_add(args: Vec<String>) -> io::Result<()> {
    let repo = open_home_repo()?;
    let mut index = match repo.index() {
        Ok(index) => index,
        Err(e) => {
            eprintln!("Could not open index: {}", e);
            exit(74);
        }
    };
    let args = AddArgs::new(args);
    match args.mode {
        AddMode::All => {
            let mut files_to_update = Vec::new();
            let mut options = StatusOptions::new();
            options.include_untracked(false);
            options.show(git2::StatusShow::Workdir);
            let status = match repo.statuses(Some(&mut options)) {
		Ok(status) => status,
		Err(err) => {
		    eprintln!("Could not get repo status: {}", err);
		    exit(74);
		}
	    };
            if status.len() > 0 {
                for i in status.iter() {
                    files_to_update.push(match i.path() {
                        Some(path) => {
			    let path = Path::new(path);
			    if let Err(e) = index.add_path(path) {
				eprintln!("index error: {}", e);
				exit(74);
			    }
			 
			}
                        None => {
                            eprintln!("Path is not valid utf-8");
                            exit(1);
                        }
                    })
                }
		if let Err(e) = index.write() {
		    eprintln!("could not write to index: {}", e);
		    exit(74);
		}
            }

        }
        AddMode::Normal => {
            let files: Vec<&Path> = args.values.iter().map(|x| Path::new(x)).collect();
            for i in files {
                if let Err(e) = index.add_path(i) {
                    eprintln!("index error: {}", e);
                    exit(74);
                }

            }
	    if let Err(e) = index.write() {
                eprintln!("could not write to index: {}", e);
                exit(74);
            }
        }
    };

    Ok(())
}

/// Commits current index to HEAD.
pub fn run_initial_commit(args: &Vec<String>) -> io::Result<()> {
    let message: &str = if args.len() != 0 {
        &args[0]
    } else {
        print_commit_usage();
        exit(64);
    };

    let repo = match open_home_repo() {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Unable to open repo: {}", e);
            exit(74);
        }
    };

    let (sig, tree) = gen_init_comimt_args(&repo)?;
    let _commit = match repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[]) {
        Ok(id) => id,
        Err(_err) => {
            eprintln!("Could not create commit");
            exit(74);
        }
    };
    Ok(())
}

pub fn run_commit_action(args: &Vec<String>) -> io::Result<()> {
    let message: &str = if args.len() != 0 {
        &args[0]
    } else {
        print_commit_usage();
        exit(64);
    };

    let repo = match open_home_repo() {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Unable to open repo: {}", e);
            exit(74);
        }
    };

    let (parent, sig, tree) = gen_commit_args(&repo)?;
    let parent = match parent.as_commit() {
        Some(commit) => commit,
        None => {
            eprintln!("could not get parent commit");
            exit(74);
        }
    };
    let _commit = match repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent]) {
        Ok(id) => id,
        Err(_err) => {
            eprintln!("Could not create commit");
            exit(74);
        }
    };
    Ok(())
}

/// Runs the program in log mode
pub fn run_log() -> io::Result<()> {
    let repo = open_home_repo()?;
    let head = match repo.head() {
        Ok(head) => head,
        Err(e) => {
            eprintln!("Unable to get HEAD: {e}");
            exit(74);
        }
    };
    let commit = match head.peel_to_commit() {
        Ok(commit) => commit,
        Err(e) => {
            eprintln!("Unable to get commit from HEAD: {e}");
            exit(74);
        }
    };
    let sha = commit.id();
    let author = commit.author();
    let time = {
        let from = commit.time().seconds();
        let utc_val = Local.timestamp(from, 0).to_string();
        utc_val
    };
    let message = match commit.message() {
        Some(message) => message,
        None => "",
    };
    println!("commit {sha}");
    println!("Author: {author}");
    println!("Date: {time}");
    println!();
    println!("   {message}");
    println!();
    Ok(())
}

/// Runs the program in commit mode.
pub fn run_commit(args: Vec<String>) -> io::Result<()> {
    let repo = open_home_repo()?;
    let args = CommitArgs::new(args);
    match args.mode {
        CommitMode::Commit => {
            let x = if let Ok(_) = repo.revparse_ext("HEAD") {
                run_commit_action(&args.values)
            } else {
                run_initial_commit(&args.values)
            };
            x
        }
    }
}

/// Initializes a new git home directory.
pub fn run_init() -> io::Result<()> {
    let canonical_path = match resolve_git_repo() {
        Ok(string) | Err(string) => string,
    };
    let git_home_path = Path::new(&canonical_path);

    match Repository::init_bare(&git_home_path) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Could no create git home repo: {}", e);
            exit(74);
        }
    }
}
