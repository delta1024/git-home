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

use super::{args::*, git::*, usage::*, GIT_HOME_DIR};
use chrono::{Local, TimeZone};
use git2::{Repository, StatusOptions};
use crate::args::ProgMode;
use std::boxed::Box;
use std::{env, io::{self, Write, Read}, path::Path, process::{exit, Command, Stdio}};
/// Runs the program in add mode.
pub fn run_add(args: AddArgs) -> io::Result<()> {
    let repo = open_home_repo()?;
    let mut index = match repo.index() {
        Ok(index) => index,
        Err(e) => {
            eprintln!("Could not open index: {}", e);
            exit(74);
        }
    };

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
    let repo = match open_home_repo() {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Unable to open repo: {}", e);
            exit(74);
        }
    };

    let message: String = String::from(&args[0]);

    let (sig, tree) = gen_init_comimt_args(&repo)?;
    let _commit = match repo.commit(Some("HEAD"), &sig, &sig, &message, &tree, &[]) {
        Ok(id) => id,
        Err(_err) => {
            eprintln!("Could not create commit");
            exit(74);
        }
    };
    Ok(())
}
pub fn run_commit_action(args: &Vec<String>) -> io::Result<()> {
    let repo = match open_home_repo() {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Unable to open repo: {}", e);
            exit(74);
        }
    };
    let message: String = String::from(&args[0]);

    let (parent, sig, tree) = gen_commit_args(&repo)?;
    let parent = match parent.as_commit() {
        Some(commit) => commit,
        None => {
            eprintln!("could not get parent commit");
            exit(74);
        }
    };

    let _commit = match repo.commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&parent]) {
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
    let args = CommitArgs::new(args)?;
    if args.values[0] == "" {
	eprintln!("Commit aborted");
	exit(1);
    }
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

pub fn run_passthrough(prefix_args: Option<Box<ProgMode>>, args: Vec<String>) -> io::Result<()> {
    if let Some(prefix_args) = prefix_args {
	match *prefix_args {
	    ProgMode::Add(args) => run_add(args)?,
            ProgMode::Init => run_init()?,
            ProgMode::Status(color) => print_repo_status(color)?,
            ProgMode::Commit(args) => run_commit(args.values)?,
            ProgMode::Log => run_log()?,
            ProgMode::Help | ProgMode::Passthrough(_,_) => print_usage()?,
	    ProgMode::None => (),
	}
    };
    
    let home_dir = env::var("HOME").unwrap_or_else(|err| {
	eprintln!("unable to get value of $HOME: {}", err);
	exit(74);
    });
    let git_dir = env::var("GIT_HOME_DIR").unwrap_or(GIT_HOME_DIR.to_string());

    let mut git = Command::new("git")
        .args(&[
            "-C",
	    &home_dir,
            "--work-tree",
            ".",
            "--git-dir",
	    &git_dir,
            "-c",
            "status.showUntrackedFiles=no",
	    "--no-pager"
        ])
        .args(args)
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Could not spawn process.");
    match git.wait() {
	Ok(status) => match status.code() {
	    Some(code) => exit(code),
	    None => {
		println!("Git terminated by signal");
		exit(0)
	    }
	}
	Err(err) => {
	    eprintln!("{}", err);
	    exit(74)
	}
    }

}
