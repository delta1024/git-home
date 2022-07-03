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
use super::{args::*, git_functions::*, GIT_HOME_DIR};
use git2::Repository;
use std::{env, io, path::Path, process::exit};

/// Runs the program in add mode.
pub fn run_add(args: Args) -> io::Result<()> {
    let repo = open_home_repo()?;
    let mut index = match repo.index() {
        Ok(index) => index,
        Err(e) => {
            eprintln!("Could not open index: {}", e);
            exit(74);
        }
    };
    let files: Vec<&Path> = match args.add_values {
        Some(ref vec) => vec.iter().map(|x| Path::new(x)).collect(),
        None => panic!("Expectred options"),
    };

    for i in files {
        if let Err(e) = index.add_path(i) {
            eprintln!("index error: {}", e);
            exit(74);
        }
        if let Err(e) = index.write() {
            eprintln!("could not write to index: {}", e);
            exit(74);
        }
    }

    Ok(())
}

/// Initializes a new git home directory.
pub fn run_init() -> io::Result<()> {
    println!(
        "git home will create a repository at $HOME/{} by default.",
        GIT_HOME_DIR
    );
    println!("You can change this behavior by seting the value of GIT_HOME_DIR in your shell startup file.");
    print!("continue? (y/n) ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    // Remove the '\n' at the end.
    input.pop();

    if input == "y" {
        let home_dir = match env::var("HOME") {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Couldn't get $HOME value: {}", e);
                exit(74);
            }
        };
        let git_dir = match env::var("GIT_HOME_DIR") {
            Ok(val) => val,
            Err(_e) => GIT_HOME_DIR.to_string(),
        };
        let canonical_path = format!("{}/{}", home_dir, git_dir);
        let git_home_path = Path::new(&canonical_path);

        match Repository::init_bare(&git_home_path) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Could no create git home repo: {}", e);
                exit(74);
            }
        }
    } else {
        exit(0);
    }
}
