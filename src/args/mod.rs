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

use std::env;
use std::fmt::Debug;
use std::default::Default;
use std::path::{Path, PathBuf};
use std::process::exit;
pub mod usage;
use usage::{print_add_help, print_commit_usage};
#[derive(Debug, PartialEq)]
pub enum CommitMode {
    Commit,
}

#[derive(Debug)]
pub struct CommitArgs {
    pub mode: CommitMode,
    pub values: Vec<String>,
}

impl CommitArgs {
    pub fn new(args: Vec<String>) -> CommitArgs {
	let mut args = args.iter();
	let temp_args = match args.next() {
	    Some(val) => String::from(&*val),
	    None => {
		print_commit_usage();
		exit(64);
	    }
	};
	let mode;
	let values: Vec<String>;
	if  temp_args.len() > 10 && &temp_args[0..9] == "--message" {
	    let mut temp_args = temp_args.split('=');
	    temp_args.next();
	    values = temp_args.map(|x| String::from(x)).collect();
	    mode = CommitMode::Commit;
	} else if temp_args == "-m"{
	    values = args.map(|x| String::from(x)).collect();
	    mode = CommitMode::Commit;

	} else {
	    print_commit_usage();
	    exit(64);
	};
	
	CommitArgs {
	    mode, 
	    values,
	}
    }
}


/// Specifies which mode the program will run in.
#[derive(Debug, PartialEq)]
pub enum ProgMode {
    Add,
    Init,
    Status,
    Commit,
    Log,
    Help,
    None,
}

impl Default for ProgMode {
    fn default() -> ProgMode {
        ProgMode::None
    }
}

/// Holds the program paramaters.
pub struct Args {
    pub mode: ProgMode,
    pub values: Vec<String>,
}

impl Args {
    pub fn new(mode: ProgMode, values: Vec<String>) -> Args {
	Args {
	    mode,
	    values,
	}
    }
}

fn canonicalize_file_path<'a>(init_path: &str) -> String {
    match Path::new(init_path).canonicalize() {
        Ok(paths) => {
            let mut path_iter = paths.iter();

            // Clear /root/home from PathBuf
            path_iter.next();
            path_iter.next();

            let user = match path_iter.next() {
                Some(user) => user,
                None => {
                    eprintln!("Cannot use git home at top level of file system.");
                    exit(74);
                }
            };
            let env_user = match env::var("USER") {
                Ok(u) => u,
                Err(_e) => panic!("$USER not set."),
            };
            if user.to_str().unwrap() != env_user {
                eprintln!("git home should only be used on files in the users own home directory");
                exit(64);
            }

            let buf: PathBuf = path_iter.collect();
            let paths = buf.as_path();
            String::from(match paths.to_str() {
                Some(s) => s,
                None => {
                    eprintln!("could not convert path to string");
                    exit(74);
                }
            })
        }

        Err(e) => {
            eprintln!("Couldn't canonicalize path: {}", e);
            exit(74);
        }
    }
}

/// Returns the formated progam arguments
pub fn format_args() -> Args {
    let mode;
    let mut prog_args = env::args();
    // Clear the binary location from the arguments iterator.
    prog_args.next();

    let temp_mode = match prog_args.next() {
        Some(mode) => mode,
        None => "".to_string(),
    };

    if &temp_mode == "add" {
        mode = ProgMode::Add;
      } else if &temp_mode == "init" {
        mode = ProgMode::Init;
        if let Some(_val) = prog_args.next() {
            eprintln!("home init takes no args.");
            exit(1);
        }
    } else if &temp_mode == "status" {
        mode = ProgMode::Status;
    } else if &temp_mode == "commit" {
        mode = ProgMode::Commit;
    } else if &temp_mode == "log" {
	mode = ProgMode::Log;
    } else if &temp_mode == "--help" {
        mode = ProgMode::Help;
    } else {
        mode = ProgMode::None;
    }

    let values: Vec<String> = match mode {
	ProgMode::Add => {
	    let arg_opts: Vec<String> = prog_args.map(|x| canonicalize_file_path(&x)).collect();
            if arg_opts.len() == 0 {
		print_add_help();
		exit(64);
            }
	    arg_opts
	}
	_ => prog_args.collect(),

    };
    Args::new(mode, values)
}
