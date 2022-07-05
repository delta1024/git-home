use crate::git::resolve_git_repo;
use crate::GIT_HOME_DIR;
use std::io;

pub fn print_commit_usage() {
    println!("Usage: ");
    println!("\tgit home commit <command>");
    println!();
    println!("Options: ");
    println!("\t[-m | --message=]\"messagd\":");
    println!("\t\t Commits index to working head with message."
	     
    )

	
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
    print!("\t");
    print_add_help();
    println!("\tgit home commit <args>");
    println!("Commands:");
    println!("\t    add: add a file to the git_home repo.");
    println!("\t status: print staus of files in the index.");
    println!("\t   init: initialize a new home repo.");
    println!("\t commit: commit current index to repository.");
    println!("\t    log: prints a log of the last commit.");
    println!("\t --help: prints this help dialog.");
    println!();
    println!("Global Variables:");
    println!("\tGIT_HOME_DIR: {}", git_dir);

    Ok(())
}
