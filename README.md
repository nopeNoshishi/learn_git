# Understanding Git through images

I am a newbie, still a few months into my career as a developer in Japan. I was inspired by [Nico Riedmann's Learn git concepts, not commands](https://dev.to/unseenwizzard/learn-git-concepts-not-commands-4gjc), and I have summarized git in my own way. Of course, I supplemented it with reading the [official documentation ](https://git-scm.com/doc)as well.
Understanding git from its system structure makes git more fun. I have recently become so addicted to git that I am in the process of creating my own git system.

<!-- TOC -->

- [Whati is Git?](#what-is-git)
	- [Manage versions and Distribute work](#manage-distribute)
	- [Using Git means](#using-git-means)
	- [Understanding by image](#understading-by-image)
- [Start new work](#start-new-work)
	- [Repositories](#repositories)
	- [Copy the repository and start working](#copy-the-repository)
	- [(Supplemental) Working Directory](#working-directory)
	- [Change and Add files](#change-and-add-file)
	- [Adapt to remote repositories](#adapt-to-remote)
	- [View Differences](#view-differences)
	- [(Aside) One step called staging area](#staging-area)
	- [Summary](#summary1)
- [Branch](#branch)
	- [Create new branch](#create-new-branch)
	- [Work in Branches](#work-in-branches)
	- [(Aside)Git-Flow and GitHub-Flow](#gitflow-githubflow)
	- [Summary](#summary2)
- [Merge](#merge)
	- [Fast Forward](#fast-forward)
	- [No Fast Forward](#no-fast-forward)
	- [Deal with Conflicts](#deal-with-conflicts)
	- [Delete unnecessary branches](#delete-unnecessary-branches)
	- [(aside) What is the brunch?](#what-is-the-branch)
	- [Summary](#summary3)
- [Rebase](#rebase)
	- [Move the branch](#move-branch)
	- [Deal with rebase conflicts](#deal-with-rebase-conflicts)
- [Keep local repositories up-to-date](#keep-up-to-date)
	- [Branch and Repository](#branch-and-repository)
	- [Check the latest status](#check-the-latest-status)
	- [Update to the latest status](#update-to-the-status)
	- [Deal with pull conflicts](#deal-with-pull-conflicts)
	- [(Aside) Identity of pull requests](#identity-of-pull-requests)
- [Useful Functions](#useful-functions)
	- [Correct the commit](#correct-the-commit)
	- [Delete the commit](#delete-the-commit)
	- [Evacuate the work](#evacuate-the-work)
	- [Bring the commit](#bring-the-commit)
	- [Mastering HEAD](#mastering-head)
- [End](#end)
	- [Source code management without Git](#source-code-managemaent-without-git)
	- [Where is the remote repository](#where-is-the-remote-repository)
	- [Pointer](#pointer)
	- [To further understand Git](#to-further-understading-git)
- [Reference](#references)

<!-- TOC -->

<a id="markdown-what-is-git" name="what-is-git"></a>
## Whati is Git?

<a id="markdown-manage-distribute" name="manage-distribute"></a>
### Manage versions and Distribute work
Git is a type of source code management system called a distributed version control system.
Git is a tool to facilitate development work by **recording and tracking the changelog (version) of files**, comparing past and current files, and clarifying changes.
The system also allows **multiple developers to edit files at once**, so the work can be distributed.

<a id="markdown-using-git-means" name="using-git-means"></a>
### Using Git means
First, make a copy of the file or other files in a storage location that can be shared by everyone (from now on referred to as "remote repository") on your computer (from now on referred to as "local repository"), and then add or edit new code or files.
Then, the files will be updated by registering them from the local repository to the remote repository.

<img width="600" alt="retool.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/22c6b2e3-aeda-44c6-8f54-3b7e80db129b.png">


<a id="markdown-understading-by-image" name="understading-by-image"></a>
### Understading by image
When dealing with Git, it is important to follow "how to work" from "what" to "what".
If you only operate commands, you may not understand what is happening and use the wrong command.

**(info)**
When manipulating Git, try to imagine what is happening before and after the operation.


<a id="markdown-start-new-work" name="start-new-work"></a>
## Start new work

<a id="markdown-repositories" name="repositories"></a>
### Repositories
A repository in Git is a storage for files, which can be remote or local.

**Remote Repository** is a repository where the source code is placed on a server on the Internet and can be shared by everyone.
**Local repository** is a repository where the source code is located on your computer and only you can make changes.


<a id="markdown-copy-the-repository" name="copy-the-repository"></a>
### Copy the repository and start working
First, prepare your own development environment.
All you need only to do is decide in which directory you will work.
For example, your home directory is fine, or any directory you normally use.

Next, copy and bring the files from the remote repository.
This is called `clone`.

<img width="450" alt="clone.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/36772bce-9111-1f50-bafd-97246035e78e.png">

The remote repository called `project` contains only `first.txt`, and this is the image when you `clone` the remote repository.

**(info)**
Of course, you may create a local repository first and then reflect the remote repository.
This is called `initialize` and allows you to convert a directory you are already working on into a repository.


<a id="markdown-working-directory" name="ワーキングディworking-directoryレクトリ"></a>
### (Supplemental) Working Directory
A working directory is not any special directory, but a directory where you always work on your computer.
It's easier to understand if you think of it as a directory where you can connect to the target directory that Git manages (in this case, `project`) with a Git staging area or local repository.

<img width="400" alt="clone3.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/162c1db8-8271-fd4e-f702-2f77a0329564.png">

<a id="markdown-change-and-add-file" name="change-and-add-file"></a>
### Change and Add file
Changes to the source code are made through the working directory, the staging area.
Actually, in the working directory, we work.

Let's create a new file called `second.txt`.

<img width="450" alt="create_file.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/8361cfea-fca9-3193-835b-e31b449df6b8.png">

Next, move the modified file to the staging area.
This is called `add`.

It is a feature of Git that there is a cushion before changes are reflected in the local repository.
I will explain why this cushion exists in more detail later.

<img width="450" alt="add.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/e975ecbf-c65f-f9ab-6dec-75984de363d6.png">

Then, we registere the content in the staging area to the local repository.
This is called `commit`.

By the way, we can comment when you `commit`.
In this case, we added a file, so write `git commit -m 'add second.txt'`.

<img width="450" alt="commit.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/f8908fd7-72b1-51d7-4aa6-c742b56a4fac.png">


**(info)**
When you commit, a **commit object** is created in the repository.
A simple explanation of a commit object is the data that has the updater's information and the modified file.
(All data is saved, not just the differences, but the entire state of the file at that moment (snapshot).
Please refer to [Git Objects](https://git-scm.com/book/en/v2/Git-Internals-Git-Objects) for more information about Git objects.


<a id="markdown-adapt-to-remote" name="adapt-to-remote"></a>
### Adapt to remote repositories
Then, the work is done!
The last step is to reflect the changes in the local repository to the remote repository.
This is called `push`.

<img width="400" alt="push.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/d57789fe-08f2-3234-ec5a-bf1457a6cb6d.png">

It may be easier to understand if you think of it as a commit to a remote repository.

<a id="markdown-view-differences" name="view-differences"></a>
### View Differences
Changes between the same file are called `diff`.
We can see the changing points in the file.

I won't go into the details of the commands, but here are three that I use frequently.
`git diff --stage` to see the changes from the original working directory before you `add`.
`git diff --stage` to see changes to the working directory after `add`.
`git diff <commit> <commit>` to compare commits.


<a id="markdown-staging-area" name="staging-area"></a>
### (Aside) One step called staging area
As development work grows, we often make many changes in one working directory.
What happens if you put all the changes in a local repository at once?
In this case, when parsing the commits, you may not know where a feature was implemented.

In Git, it is recommended to do one `commit` per feature.
This is why there is a staging area where you can subdivide the `commit` unit into smaller units.

<img width="400" alt="push.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/5b25c70d-baf5-0913-d220-f60aedf583ea.png">


The concept of Git is to stage only what is needed, and then proceed with the work or `commit` ahead of time to promote efficient development that can be traced back through the history of each implementation.

<a id="markdown-summary1" name="summary1"></a>
### Summary
The basic workflow is to `clone` once and then `add`, `commit`, and `push` for each working.

![basic.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/9dccfc0e-f2d9-5405-29c4-890f34894308.gif) 


**(info)**
`clone`: Make a copy from the remote repository to your development environment (local repository and working directory).
`add`: Add files from the working directory to the staging area and prepare them for commit.
`commit`: Register the file from the staging area to the local repository. At this time, a commit object is created.
`push`: Register changes from the local repository to the remote repository.


<a id="markdown-branch" name="branch"></a>
## Branch
We create a `branch` to change and add files in multiple branches.
The files saved in the `main` branch are in ongoing use.
The reason for the separate branches is to work **without affecting the currently running source code**.


<a id="markdown-create-new-branch" name="create-new-branch"></a>
### Create new branch
Let's create the branch called `develop`!
We can create a branch with `git branch <new branch>` or `git checkout -b <new branch>`.
The former just create a branch, the latter create a branch and moves you to that branch.
(Branches are maintained in the repository.)


<img width="450" alt="cretae_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a7f499f9-bf5e-0c58-1a35-59cd3e2a4c30.png">

The key point when generating branches is **which branch to derive from**.
We can specify the source as `git checkout -b <new branch> <from branch>`.
If we don't, the branch you are currently working on becomes the `<from branch>`.


**(info)**

A branch is actually a **pointer** to the commit (strictly speaking, a hash of commit objects).
Generating a new branch means that the new branch indicate to the commit that the from　branch pointed to as well.

<a id="markdown-work-in-branches" name="work-in-branches"></a>
### Work in Branches
Moving the branch is called `checking out`.
The pointer to the branch you are currently working on is called `HEAD`.
So, moving from the `main` branch to the `develop` branch means changing the `HEAD`.

<img width="450" alt="checkout_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/3a7379b1-7a1c-35ba-02be-619d13a192ad.png">

Now both branches point to the commit named `Atr3ul`.
You just added `second.txt` by committing in the `main` branch, so you are ahead of the commit `f27baz`.
From here, let's say you change `second.txt` in the `develop` branch and make a new commit.


<img width="450" alt="in_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/1ae0aa22-4c8d-411b-b20e-c29047dcdb4d.png">


Then, as shown in the figure, the `develop` branch created a commit called `m9sgle` and pointed to that commit.

The current HEAD position (working branch position), what stage the file has been worked on, or the status of who is working on it is called `status`.


**(info)**

If you are familiar with object-oriented, you may understand the reason for the arrow on the commit.
It represents the relationship between a "parent" commit and a "child" commit.
The assumption is that `parent←-child`, that is, how much the child (commit) born from the parent (commit) has grown (changed).

<a id="markdown-gitflow-githubflow" name="gitflow-githubflow"></a>
### (Aside)Git-Flow and GitHub-Flow
The way branches to manage will vary on development team.
On the other hand, like programming naming conventions, there is a general model for how to grow branches in Git.
Here are two simple ones. I think it's enough to know that there is such a thing.

<br>
<br>

The "Git Flow" is a fairly complex and intricate structure.
I think it's a model of how Git should be used.

<img width="450" alt="git_flow.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a05bf11e-8ad0-e82f-e5f4-76498f4b5c46.png">

**Definition of each branch**.

`master`: Branch to release a product. No working on this branch.

`development`: Branch to develope a product. When ready to release, merge to `release`. No working on this branch.

`feature`: Branch for adding features, merged into development when ready for release.

`hotfix`: For urgent post-release work (critical bug fixes, etc.), branch off from master, merge into master, and merge into develop.

`release`: For preparation of product release. Branch from `develop` with features and bug fixes to be released.
When ready for release, merge to master and merge to develop.

<br>
<br>

The "GitHub Flow" is a somewhat simplified model of the Git Flow.

<img width="450" alt="github_flow.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/b717c9a0-7612-906f-2f10-3bf7a600e755.png">

As you can see, it consists of only `master` and `feature`.
The important difference is the cushion of `pull requests` (explained in the pull below), which allows integration between branches.

<a id="markdown-summary2" name="summary2"></a>
### Summary
Basically, since there is no work on main (master), we create a branch for each work unit we want to do and create a new commit.

![branch_anime.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/f5158fea-ea6c-a033-11f7-27a9b152539c.gif)

**(info)**

`branch`: New pointer to the commit
`checkout`: Move `HEAD` to change the `branch` to work on.

<a id="markdown-merge" name="merge"></a>
## Merge
integrating the branches is called `merge`.
Basically, we merge into the `main` or `develop` branch.
Be careful not to mistake the subject of which branch is merging (absorbing) which branch.
We will always move (HEAD) to the branch from which you are deriving, and then do the integration from the branch from which you are deriving.

I am currently working on the `feature` branch and have created the following `third.txt`.

```text third.txt
Hello, World! I'm noshishi, from Japan.
I like dancing on house music.
```

<br>
Then We `add` and finished up to `commit`.  
<br>

<img width="450" alt="feature_commit.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/3fbb3116-4439-7753-74ec-260b043777f3.png">


<a id="markdown-fast-forward" name="fast-forward"></a>
### Fast Forward
When the `feature` branch points to the commit that can be traced back to the `develop` branch, the `develop` branch is in a `fast-forward` state.

First, move to `develop` with `checkout`.

<img width="450" alt="checkout_develop.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/d2564a3b-ab4f-32a3-edb4-5dffb38c7f89.png">

In this case, the `develop` branch has not progressed at all, so to `merge` the `feature` branch will simply move the commit forward.
In this case, the `develop` and `feature` branches share the same commit.

<img width="450" alt="merge_feature_no_conflict.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/95d4195d-a9a5-1fec-710e-85887c2f7f1f.png">

<a id="markdown-no-fast-forward" name="no-fast-forward"></a>
### No Fast Forward
What if the `develop` branch has progressed to a new commit by commit or merge?
This is called a `no fast-forward` situation.

In the `develop` branch, you have made changes to `first.txt` and have finished `commit`.
So the `develop` branch and the `feature` branch are completely split.

<img width="450" alt="develop_commi.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/d97a8652-dd94-2835-4cf2-94eed01e6353.png">

If you try to `merge` a `feature` branch from a `develop` branch, Git will check your changelog against each other.
If there are no conflicting edits, a `merge commit` is created immediately.
This is called an `automatic merge`.

<img width="450" alt="merge_feature_auto.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a81e9577-6aec-e565-71bf-0836fd883974.png">

<a id="markdown-deal-with-conflicts" name="deal-with-conflicts"></a>
### Deal with Conflicts
In the `no fast-forward` state, the differences in work content. is called `conflict`.
In this case, we must manually fix the `conflict` content and `commit`.

In the `develop` branch, we created the following `third.txt` and `committed`.

```text third.txt
Hello, World! I'm nope, from USA.
I like dancing on house music.
```
<br>

In the `develop` branch, `I'm nope, from USA`.
In the `feature` branch, `I'm noshishi, from Japan`.
The content of the first line is in conflict.

If you do a `merge` at this time, a `conflict` will occur.
Git will ask you to `commit` after resolving the `conflict`.

<img width="450" alt="conflict.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/794285e4-077a-17b1-4743-0e1f2685fba7.png">

<br>

(The branch we work on is the `develop` branch)

If you look at `third.txt` as instructed, you will see the following additions

```text third.txt　（conflict後）
 <<<<<<<< HEAD
 Hello, World! I'm noshishi, from Japan.
 =======
 Hello, World! I'm nope, from USA.
 >>>>>>>> feature
 I like dancing on house music.
```

The upper `HEAD`, separated by `=======`, represents the contents of the `develop` branch.
The lower side represents the `feature` branch.

You first considered which one to adopt, and decided to adopt the changes made in the `feature` branch this time.
The only operation then is to edit `third.txt` by hand (delete unnecessary parts).

```text third.txt　(After editing)
Hello, World! I'm noshishi, from Japan.
I like dancing on house music.
```

And the next thing you do is `add` and `commit`.
The `conflict` is resolved and a new `merge commit` is created.


<img width="450" alt="hand_merge.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/5a20b54e-33bd-a973-e201-8bbdaba8c0cc.png">

Conflicts are feared by beginners, but once you learn this, you will no longer be afraid.


**(info)**

If you `merge` and resolve the `conflict`, why not `merge` again?
When you `merge` once, the `develop` branch enters the `merge` state, and if there are no `conflicts`, the new files are automatically `added` and `commit`.
So it is not a special `commit` after `conflict` is resolved.
That's why it's called `merge commit`.

<a id="markdown-delete-unnecessary-branches" name="delete-unnecessary-branches"></a>
### Delete unnecessary branches
The merged branch is basically useless, so we will delete it.
If we leave a branch alone, you can move from the branch you want to delete to another branch and `git branch -d <branch>`.
You may think the commits on that branch are deleted.
In fact, the commits are carried over to the merged branch.
You can use `git log` to see all the commits you've made on the branch and the commits on the merged branch.

<a id="markdown-what-is-the-branch" name="what-is-the-branch"></a>
### (Aside) What is the brunch
We said that a branch is a pointer to a commit, but it also holds another important data.
It is all the commits that have been made on that branch.

A branch is a collection of commits, and it has a pointer to the latest commit in that collection. (Strictly speaking, the commit can trace back to previous commits.)

The following diagram illustrates this.

<img width="450" alt="branch_image.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/8cbb8c69-d048-778c-9d2d-db1f3be3b3be.png">

So we can think of branches on a horizontal axis like Git Flow.
By the way, if you draw the above diagram with branches on the horizontal axis, it looks like this.

<img width="450" alt="branch_image2.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/97ed7459-a4ee-afd9-1e2e-d7a0ffbf8f1e.png">

<a id="markdown-summary3" name="summary3"></a>
### Summary
`fast-forward merge`
![fast_merge.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/4abed1f6-7431-50d5-b90a-58e152633096.gif)

`no fast-forward merge`
![nofast_merge1.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/3a29143b-fe53-3df7-352c-ccd569373685.gif)

`no fast-forward merge with conflict`
![nofast_merge2.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/2898da3a-dc15-e1fd-c991-61d80ca0f6a1.gif)


**(info)**

`merge`: To integrate (absorb) a working branch (such as `feature`) into a specific branch (such as `main` or `develop`) and create a new commit.


<a id="markdown-rebase" name="rebase"></a>
## Rebase
各ブランチの派生元のコミットを変えてブランチ同士を統合することを`rebase`と言います。
`merge`に似ていますが、異なる点は、作業を行うブランチが 「派生先」ブランチということです。

`develop`ブランチと`feature`ブランチで作業していているとしましょう。

<img width="450" alt="base_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/0655a75b-4466-a069-0ad7-f98a2785996c.png">

<a id="markdown-move-branch" name="move-branch"></a>
### Move the branch
`develop`ブランチの現在のコミット、`feature`ブランチに反映させるためには、`feature`ブランチが派生した`gp55sw`コミットから`3x7oit`コミットに移動させる必要があります。

これは`feature`ブランチから`git rebase develop`とすることで一気に移動できます。

<img width="450" alt="move_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/acecd79a-66eb-1836-f911-a76b2ad3dbef.png">


`merge`をしているというより、`develop`ブランチの最新のコミットから`feature`ブランチを生やし直すに近いです。
ただし、コミットごと移動し、新しいコミットを行うことが違いです。

<img width="450" alt="rebase_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/d37b85ab-7b78-8a68-3302-6081edc036b1.png">

なぜ、こんな移動（統合）をするのかというと、一つは　`fast-forward`になりいつでも`merge`を行いやすいことです。
もう一つは、コミットが一直線になることで容易にコミット履歴を辿れ、ファイルの更新順に整合性を持たせることができるためです。

<a id="markdown-deal-with-rebase-conflicts" name="deal-with-rebase-conflicts"></a>
### Deal with rebase conflicts
もちろん`rebase`にも`conflict`が存在します。
上記の場合、`feature`ブランチでは、`fourth.txt`を追加しましたが、その後`develop`ブランチでのコミットでは、`fourth.txt`に関わる変更がないため、`conflict`は起こりません。

ですが、以下のように変更内容が被っていた場合、`conflict`が起こります。

<img width="450" alt="rebase_conflict.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/c04e5c00-7523-ccfd-9311-421fe252fdc0.png">

でも`merge`と同じように対処すれば大丈夫です。
ただし、差分を確認してファイルの編集を終えたら、`git rebase --continue`で作業を終わりましょう。
`commit`しなくても自動でコミットしてくれます。


**(info)**

`rebase`:ブランチの派生元であるコミットを移動させて新しいコミットを行うこと。


<a id="markdown-keep-up-to-date" name="keep-up-to-date"></a>
## Keep local repositories up-to-date
ローカルである程度作業を進めると、リモートリポジトリが他の開発者によって更新されている場合があります。
この場合において、リモートリポジトリの情報を再度ローカルリポジトリに反映させるために行うのが`pull`です。

<a id="markdown-branch-and-repository" name="branch-and-repository"></a>
### Branch and Repository
ブランチは、各リポジトリに保存されています。
実際に作業を行うブランチです。

<img width="450" alt="brancha.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/76f507a4-232e-894e-e01f-328ab7138577.png">

一方で、ローカルリポジトリには、リモートリポジトリをコピーしたブランチがあります。
これは「リモート追跡ブランチ」と呼ばれます。
`remotes/<remote branch>`でリモートのブランチと紐づく名前のブランチです。

これは、あくまでもリモートリポジトリを監視しているに過ぎません。

<img width="450" alt="remotes.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/9c014980-79e5-bba6-c3d8-c8f6a237178d.png">


<a id="markdown-check-the-latest-status" name="check-the-latest-status"></a>
### Check the latest status
リモートリポジトリの`develop`ブランチがリモート追跡ブランチより一つ進んでいる状況だったとします。。

<img width="450" alt="pull_notupdate.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/3393d6c6-1a65-31c7-7afb-f98170f79872.png">

リモートリポジトリのブランチの最新の状況をリモート追跡ブランチに反映させることを`fetch`と言います。

<img width="450" alt="fetch_update.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/ff486c7c-963d-5962-0878-a144aab5893f.png">


<a id="markdown-update-to-the-status" name="update-to-the-status"></a>
### Update to the latest status
もう少し踏み込んで、ローカルブランチにも反映させたい場合、`pull`を行います。
`pull`するとまず、ローカルのリモート追跡ブランチが更新されます。
その後にローカルブランチに`merge`を行います。

<img width="450" alt="pull_update.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a9526959-34d0-20fd-d128-4b5b37f19298.png">


今回は、`develop`ブランチの一つ先に進んだコミットがあったので、ローカルブランチの`develop`ブランチに`merge`して新たなコミットが作成されました。

<a id="markdown-deal-with-pull-conflicts" name="deal-with-pull-conflicts"></a>
### Deal with pull conflicts
リモートリポジトリのコミットで行われた変更と、ローカルリポジトリのコミットで行った変更が競合してしまった時、`pull`したときにリモート追跡ブランチとローカルブランチで`conflict`が起こります。下記の場合、`remotes/develop`と`develop`ブランチが競合しています。

<img width="450" alt="pull_conflict.png" src="">

`push`は、`fetch`と`merge`なので、`merge`の`conflict`と同じ対処方法で解決できます。
今回は`develop`が`remotes/develop`を`merge`するので、作業ブランチは`develop`です。
原因のフォルダを開いて、修正したら`commit`を行いましょう。


<a id="markdown-identity-of-pull-requests" name="identity-of-pull-requests"></a>
### (Aside) Identity of pull requests
基本的にリモートとローカルの関係は、リモートリポジトリからローカルリポジトリにpullし、ローカルリポジトリからリモートリポジトリにpushすることになります。
ですが、GitHubをはじめとするサービスには、ローカルリモートリポジトリ内にあるブランチから、mainブランチのようなブランチにmergeする前にrequestを送るという仕組みをとっています。(※12/9追記)ローカルから直接反映する訳ではありません。
というのは、開発者の個人の判断mainブランチなどにpushして、リモートリポジトリを更新してしまうと誰もチェックできずに大きな障害が発生する可能性があります。
一旦上位の開発者がコードをレビューするプロセスを挟むのがpull requestです。

<img width="450" alt="pull_request.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/49ceb654-292c-0b89-f537-1a771be1a7cd.png">

**(info)**

`pull`:`fetch` + `merge`。`pull`は、リモートリポジトリの状態をローカルリポジトリに反映させること。


<a id="markdown-useful-functions" name="useful-functions"></a>
## Useful Functions

<a id="markdown-correct-the-commit" name="correct-the-commit"></a>
### Correct the commit
前回行ったコミットを訂正するための`commit`することを`revert`と言います。
例えば`m9sgLe`で`second.txt`をローカルリポジトリに追加したとしましょう。

`revert`を行うと、その`commit`が取り消されて`second.txt`はローカルリポジトリから無くなります。

<img width="450" alt="revert.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/255417d6-a05f-b68c-89eb-436ec1b92c7c.png">

`revert`の良さは、`commit`を残せることです。後ほど紹介する`reset`と区別しましょう。

<a id="markdown-delete-the-commit" name="delete-the-commit"></a>
### Delete the commit
現在の最新のコミットを取り消してもう一度作業することを`reset`といいます。

`--soft`オプションを使用すると`add`した直後に戻ることができます。
`--mixed`オプションを使用すると、ワーキングディレクトリで作業していた段階に戻ることができます。
`--hard <commit>`オプションを使用すると、戻るコミット地点までのすべてのコミットを削除し、指定コミットに`HEAD`を移動させます。

<img width="450" alt="reset.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/6d4a7eee-ce0a-4af5-ef0f-5541cb6b66b2.png">


`reset`は**完全にコミットが削除**されるので、特に'--hard'オプションについては、余程のことがなければ使用しないことを推奨します。


<a id="markdown-evacuate-the-work" name="evacuate-the-work"></a>
### Evacuate the work
変更ファイルがあると他のブランチに移動できないので、`commit`までするか変更を破棄してしまうかを選択しなければなりません。
そんな時に活躍するのが`stash`です。
ワーキングディレクトリやステージングエリアにあるファイルを一時避難させることができます。

<img width="450" alt="stash.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a8e83f88-bce5-560b-d496-80bea2918a65.png">

他のブランチに移動したい時、`stash`し、帰ってきたら`stash pop`で避難したファイルを取り戻して作業を再開します。


<a id="markdown-bring-the-commit" name="bring-the-commit"></a>
### Bring the commit
任意のコミットを現在のブランチに持ってきてコミットを作ることを`cherry-pick`と言います。
まさにいいとこ取りのような機能です。

<img width="450" alt="cherry.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/27601d74-62c8-2814-dfc6-3e31e6abf246.png">


以前に`feature`ブランチで実装した**〇〇〇な機能だけ**持ってきて、現在`develop`ブランチの作業に使用したいときなどに使用します。

<a id="markdown-mastering-head" name="mastering-head"></a>
### Mastering HEAD
HEADは、現在作業中のブランチのポインタと説明しました。
また、ブランチはコミットを指すポインタだ、とも説明しました。

下の図を見てください。

<img width="450" alt="head.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/b4c0651f-a613-f858-9f53-7bf9de227f5d.png">

HEADが指すものは`develop`ブランチ、`develop`ブランチが指すもの`eaPk76`というコミットです。
つまり、この状況でのHEADは、`eaPk76`のコミットを指しているということになります。

よくGitのドキュメントや記事などに、コマンドの後ろに`HEAD`を使うのを見たことがありませんか？
例えば、`git revert HEAD`など。
これは、`HEAD`をコミットと置き換えることができるから実現できるコマンドなのです。



<a id="markdown-end" name="end"></a>
## End

<a id="markdown-source-code-managemaent-without-git" name="source-code-managemaent-without-git"></a>
### Source code management without Git
Mercurial has the same history as Git.
Mercurial has a very simple command line interface (CLI) that sacrifices the flexibility of Git.
Recently, based on Mercurial, Meta released a new source code management system called Sapling as open source.
I would like to try it again and write about my impressions.

<a id="markdown-where-is-the-remote-repository" name="where-is-the-remote-repository"></a>
### Where is the remote repository
A hosting service is a service that rents a server for a remote repository.
Typical examples are GitHub, Bitbucket, and Aws Code Commit for private use.
Git and Git Hub are completely different.
By the way, as mentioned above, we can use our own servers for remote repositories.

<a id="markdown-pointer" name="pointer"></a>
### Pointer
If you have been exposed to programming that deals directly with memory, such as the C programming language, you will somehow know what a "pointer" is.
On the other hand, for a beginning programmer, it seems very vague.

I said that commit objects are stored in the repository.
If there are many commit objects in the repository, how can you select the one you want?

We need a label (address) to locate a particular commit object.

<img width="600" alt="pointer.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/1d4f5378-f935-7703-b5bb-93ac76b73b28.png">

The "pointer" is a valuable data that indicates us to the label so that we don't forget it.

The label, by the way, is converted into a mysterious string through a `hash function`.
If you are curious, please refer to [How does Git compute file hashes?](https://stackoverflow.com/questions/7225313/how-does-git-compute-file-hashes).

<a id="markdown-to-further-understading-git" name="to-further-understading-git"></a>
### To further understand Git
There are many things I failed to mention in this article.

- The core of Git is a simple key-value type data store
- Details of the Git object that is the value
- How to relate with each objects.
I hope to fully explore this someday.

<a id="markdown-references" name="references"></a>
## References
- [Git Documentation](https://git-scm.com/doc)
- [Learn git concepts, not commands](https://dev.to/unseenwizzard/learn-git-concepts-not-commands-4gjc)
- [図解 Git](https://marklodato.github.io/visual-git-guide/index-ja.html)
