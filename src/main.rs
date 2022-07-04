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

use std::io;
use std::process::exit;
const GIT_HOME_DIR: &str = ".config/git_home";

mod args;
mod git_functions;
mod run;

use args::*;
use git_functions::*;
use run::*;

fn print_commit_usage() {
    println!("git home commit \"commit message\"");
}

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
        ProgMode::Commit => {
            let repo = open_home_repo()?;
            let x = if let Ok(_) = repo.revparse_ext("HEAD") {
                run_commit(args)
            } else {
                run_initial_commit(args)
            };
            x
        }
        ProgMode::None => {
            print_usage()?;
            exit(64);
        }
    }
}
