# Git Home
##  A dotfiles manager using git.

### Abstract 
Git Home is a git extention to allow one to easaly manage there
configuration files withouth needing to go through any symlinking or
generating from third party apps.

### Usage:
	
	git home [command] <args>
	git home add <file>
	git home commit <args>
	
### Commands:
 - add: add a file to the git_home repo.
 - status: print staus of files in the index.
 - init: initialize a new home repo.
 - commit: commit current index to repository.
 - log: prints a log of the last commit.
 - --help: prints this help dialog.

### Global Variables:
 - GIT_HOME_DIR: $HOME/.config/git_home (default value)
