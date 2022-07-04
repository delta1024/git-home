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
use super::print_add_help;
use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;

/// Specifies which mode the program will run in.
#[derive(Debug, PartialEq)]
pub enum ProgMode {
    Add,
    Init,
    Status,
    Commit,
    None,
}

impl Default for ProgMode {
    fn default() -> ProgMode {
        ProgMode::None
    }
}

/// Holds the program paramaters.
#[derive(Default, Debug)]
pub struct Args {
    pub mode: ProgMode,
    pub values: Option<Vec<String>>,
}

fn canonicalize_file_path<'a>(init_path: String) -> String {
    match Path::new(&init_path).canonicalize() {
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
    let mut args = Args::default();
    let mut prog_args = env::args();
    // Clear the binary location from the arguments iterator.
    prog_args.next();

    let mode = match prog_args.next() {
        Some(mode) => mode,
        None => "".to_string(),
    };

    if &mode == "add" {
        args.mode = ProgMode::Add;
        let arg_opts: Vec<String> = prog_args.map(|x| canonicalize_file_path(x)).collect();
        if arg_opts.len() == 0 {
            print_add_help();
            exit(64);
        }
        args.values = Some(arg_opts);
    } else if &mode == "init" {
        args.mode = ProgMode::Init;
        if let Some(_val) = prog_args.next() {
            eprintln!("home init takes no args.");
            exit(1);
        }
    } else if &mode == "status" {
        args.mode = ProgMode::Status;
    } else if &mode == "commit" {
	let arg_opts: Vec<String> = prog_args.collect();
	if arg_opts.len() == 0 {
	    args.values = None;
	} else {
	    let arg_opts: String = (&arg_opts[0]).into();
	    args.values = Some(vec!{arg_opts});
	}
	args.mode = ProgMode::Commit;
    } else {
        args.mode = ProgMode::None;
    }
    args
}
