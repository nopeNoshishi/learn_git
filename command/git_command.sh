#!/bin/bash
# The <> brackets are not needed for the actual command.
# Think of it as an assignment.

# watch help
## main command explanation
git help
## all command explanation
git help -a
## view all concept manual
git help -g
## each command helper
git help <command>
## view git basic usage by concept
git help <concept>

# init
git init

# clone
git clone <HTTP>
git clone <SHH>

# add
git add <filename>

# commit
git commit -m 'write here the comment'

# psuh
git push origin <remote branch name>

# diff
git diff

# branch
git branch <new branch name> <from branch name>

# checkout
git checkout <branch name>
git checkout -b <new branch name> <from branch name>

# merge
git merge <branch name>

# pull
git fetch
git pull origin <branch name>

# rebase
git rebase <branch name>

# revert
git revert HEAD

# reset
git reset --soft <commit>
git reset --mixed <commit>
git reset --hard <commit>

# stash
git stash
git stash pop
git stash list

# chery-picking
git chery-pick <commit>

# log
git log
git log --oneline
git log --oneline --graph

# reflog

### updating....
