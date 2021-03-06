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

use crate::git::resolve_git_repo;
use crate::GIT_HOME_DIR;
use std::io;

pub fn print_commit_usage() {
    println!("Usage: ");
    println!("\tgit home commit <command>");
    println!();
    println!("Options: ");
    println!("\t[-m | --message=]\"message\":");
    println!("\t\t Commits index to working head with message.")
}

pub fn print_add_help() {
    println!("git home add <file>");
}

pub fn print_usage() -> io::Result<()> {
    let git_dir = match resolve_git_repo() {
        Ok(string) => string,
        Err(_string) => format!("$HOME/{} (default value)", GIT_HOME_DIR),
    };

    println!("Usage:");
    println!("\tgit home [command] <args>");
    println!("Commands:");
    println!("\t    add: add a file to the git_home repo.");
    println!("\t status: print staus of files in the index.");
    println!("\t   init: initialize a new home repo.");
    println!("\t commit: commit current index to repository.");
    println!("\t    log: prints a log of the last commit.");
    println!("\t --help: prints this help dialog.");
    println!("\t     --: passes any commands following the double dashes to git.");
    println!("\t         any command preceding the double dash will be executed first.");
    println!();
    println!("\t\t For example, to commit your changes and then see a log of");
    println!("\t\t your commit history you could run:");
    println!();
    println!("\t\t\t git home commit -m \"some message\" -- status | less");
    println!();
    
    println!("Global Variables:");
    println!("\tGIT_HOME_DIR: {}", git_dir);

    Ok(())
}
