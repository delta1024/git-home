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

use std::default::Default;
use std::env;
use std::env::Args;
use std::fmt::Debug;
use std::io;
use std::path::{Path, PathBuf};
use std::process::exit;
pub mod usage;
use crate::git::{gen_commit_template, strip_commit_template};
use usage::{print_add_help, print_commit_usage};
#[derive(Debug, PartialEq)]
pub enum AddMode {
    Normal,
    All,
}

#[derive(Debug, PartialEq)]
pub struct AddArgs {
    pub mode: AddMode,
    pub values: Vec<String>,
}

impl AddArgs {
    pub fn new(args: Vec<String>) -> AddArgs {
        let mut args = args.iter();
        let temp_args = match args.next() {
            Some(val) => String::from(&*val),
            None => "".to_string(),
        };
        let mode;
        let values: Vec<String>;
        if temp_args.len() > 10 && &temp_args[0..9] == "--update" {
            values = args.map(|x| String::from(x)).collect();
            mode = AddMode::All;
        } else if temp_args == "-u" {
            values = args.map(|x| String::from(x)).collect();
            mode = AddMode::All;
        } else {
            values = args.map(|x| canonicalize_file_path(&x)).collect();
            mode = AddMode::Normal;
        };

        AddArgs { mode, values }
    }
}

#[derive(Debug, PartialEq)]
pub enum CommitMode {
    Commit,
}

#[derive(Debug, PartialEq)]
pub struct CommitArgs {
    pub mode: CommitMode,
    pub values: Vec<String>,
}

impl CommitArgs {
    pub fn new(args: Vec<String>) -> io::Result<CommitArgs> {
        let mut args = args.iter();
        let temp_args = match args.next() {
            Some(val) => String::from(&*val),
            None => {
                let template = gen_commit_template();
                strip_commit_template(edit::edit(template)?)
            }
        };

        let mode;
        let values: Vec<String>;
        if temp_args.len() > 10 && &temp_args[0..9] == "--message" {
            let mut temp_args = temp_args.split('=');
            temp_args.next();
            values = temp_args.map(|x| String::from(x)).collect();
            mode = CommitMode::Commit;
        } else if temp_args == "-m" {
            values = args.map(|x| String::from(x)).collect();
            mode = CommitMode::Commit;
        } else {
            values = vec![temp_args];
            mode = CommitMode::Commit;
        };

        Ok(CommitArgs { mode, values })
    }
}

/// Holds the program paramaters
#[derive(Debug, PartialEq)]
pub enum ProgMode {
    Add(AddArgs),
    Init,
    Status(bool),
    Commit(CommitArgs),
    Log,
    Help,
    None,
    Passthrough(Option<Box<ProgMode>>, Vec<String>)
}

impl Default for ProgMode {
    fn default() -> ProgMode {
        ProgMode::None
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
fn scan_for_passthrough(args: Vec<String>) -> (Option<Vec<String>>, Vec<String>) {
    let mut prog_args: Vec<String> = Vec::new();
    let mut pass_args: Vec<String> = Vec::new();
    let mut mode = None;

    for i in args {
	if let Some(()) = mode {
	    pass_args.push(i);
	} else {
	    if i == "--" {
		mode = Some(());
	    } else {
		prog_args.push(i);
	    }
	}
    }

    if pass_args.len() == 0 {
	(None, prog_args)
    } else {
	(Some(pass_args), prog_args)
    }
}
/// Returns the formated progam arguments
pub fn format_args() -> io::Result<ProgMode> {
    let mut mode = ProgMode::None;
    let mut prog_args = env::args();
    // Clear the binary location from the arguments iterator.
    prog_args.next();

    let (passthroughp, prog_args) = scan_for_passthrough(prog_args.collect());
    let mut prog_args = prog_args.iter();
    let temp_mode = match prog_args.next() {
        Some(mode) => mode,
        None => "",
    };

    if temp_mode == "add" {
        mode = ProgMode::Add(AddArgs::new(prog_args.map(|x| String::from(x)).collect()));
    } else if temp_mode == "init" {
        mode = ProgMode::Init;
        if let Some(_val) = prog_args.next() {
            eprintln!("home init takes no args.");
            exit(1);
        }
    } else if temp_mode == "status" {
        let has_color = match env::var("COLORTERM") {
            Ok(value) if value == "truecolor" || value == "24bit" => true,
            _ => false,
        };
        mode = ProgMode::Status(has_color);
    } else if temp_mode == "commit" {
        mode = ProgMode::Commit(CommitArgs::new(prog_args.map(|x| String::from(x)).collect())?);
    } else if temp_mode == "log" {
        mode = ProgMode::Log;
    } else if temp_mode == "--help" {
        mode = ProgMode::Help;
    }
    if let Some(vec) = passthroughp {
	if let ProgMode::None = mode {
	    mode = ProgMode::Passthrough(None, vec)	    
	} else {
	    let prefix_mode = mode;
	    mode = ProgMode::Passthrough(Some(Box::new(prefix_mode)), vec);
	}

    }
 Ok(mode)
}
