#!/bin/bash
# <>の括弧は実際のコマンドには必要ありません。
# 代入すると考えてください。

# clone
git clone <SHH>


# add
git add <filename>

# commit
git commit -m 'ここにコメントを書きます'

# psuh
git push origin <remote branch name>

# diff
git diff

# branch
git branch <new branch name> <from branch name>
git checkout -b <new branch name> <from branch name>

# checkout
git checkout <branch name>

# merge
git merge <branch name>

# pull
# 
git fetch
git pull <branch name>
git pull <branch name>

# rebase

# revert

# reset

# stash

# chery-picking

# log
