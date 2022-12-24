#!/bin/bash
# The <> brackets are not needed for the actual command.
# Think of it as an assignment.

# clone
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

### updating....