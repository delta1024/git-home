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

// this is a test ocmment
use std::io;
use std::process::exit;
const GIT_HOME_DIR: &str = ".config/git_home";

mod args;
mod git;
mod run;

use args::usage::*;
use args::*;
use git::*;
use run::*;

fn main() -> io::Result<()> {
    let args = format_args();

    match args.mode {
        ProgMode::Add => run_add(args.values),
        ProgMode::Init => run_init(),  
        ProgMode::Status(color) => print_repo_status(color),
        ProgMode::Commit => run_commit(args.values),
        ProgMode::Log => run_log(),
        ProgMode::Help => print_usage(),
        ProgMode::None => {
            print_usage()?;
            exit(64);
        }
    }
}
