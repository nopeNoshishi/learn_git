# コマンドを使わずに理解するGit

<!-- TOC -->

- [Gitとは](#Gitとは)
    - [バージョンを管理し、作業を分散する](#バージョンを管理し、作業を分散する)
    - [Gitを使うということ](#Gitを使うということ)
    - [完全理解の鍵はイメージ](#完全理解の鍵はイメージ)
- [新しい作業を始める](#新しい作業を始める)
    - [リポジトリ](#リポジトリ)
    - [リポジトリを複製して作業開始](#リポジトリを複製して作業開始)
    - [ファイルを変更・追加する](#ファイルを変更・追加する)
    - [リモートリポジトリを変更する](#リモートリポジトリを変更する)
    - [まとめ](#まとめ)
- [ブランチ](#ブランチ)
    - [新しいブランチを作る](#新しいブランチを作る)
    - [ブランチで作業を進める](#ブランチで作業を進める)
    - [(余談)Git-FlowとGitHub-Flow](#（余談）Git-FlowとGitHub-Flow)
    - [まとめ](#海砂利水魚)
- [マージ](#マージ)
    - [ファストフォワード](#ファストフォワード)
    - [ノーファストフォワード](#ノーファストフォワード)
    - [コンフリクトに対処する](#コンフリクトに対処する)
    - [不要なブランチは削除する](#不要なブランチは削除する)
    - [(余談)ブランチの正体を知りたい](#(余談)ブランチの正体を知りたい)
- [リベース](#リベース)
    - [ブランチごとごっそり移動させる](#ブランチごとごっそり移動させる)
    - [リベースのコンフリクトに対処する](#リベースのコンフリクトに対処する)
- [便利な機能](#便利な機能)
    - [リバート](#リバート)
    - [リセット](#リセット)
    - [スタッシュ](#スタッシュ)
    - [チェリーピッキング](#チェリーピッキング)
- [終わりに](#終わりに)
    - [Git以外のソースコードの管理](#Git以外のソースコードの管理)
    - [リモートリポジトリの居場所](#リモートリポジトリの居場所)
    - [さらにGitを理解するために](#さらにGitを理解するために)
- [参考サイト](#参考サイト)


<!-- /TOC -->

<a id="markdown-Gitとは" name="Gitとは"></a>
## Gitとは

<a id="markdown-バージョンを管理し、作業を分散する" name="バージョンを管理し、作業を分散する"></a>
### バージョンを管理し、作業を分散する
Gitは、分散型バージョン管理システムと呼ばれるソースコードの管理システムの1種です。Gitは、ファイルの変更履歴（バージョン）を記録・追跡したりすることで、過去と変更後のファイルを比較し、円滑に開発作業を進めるためのツールです。また、一度に複数の開発者がファイルを編集できる環境を用意しているので、作業を分散して行うことができます。

<a id="markdown-Gitを使うということ" name="Gitを使うということ"></a>
### Gitを使うということ
みんなで共有できる保存場所（以下、リモートリポジトリ）にあるファイルなどを、手元のパソコン（以下、ローカルリポジトリ）にコピーを作って、新しいコードやファイルを追加・編集して、今度は、ローカルリポジトリからリモートリポジトリへ登録することでファイルを更新していくこと。

<img width="600" alt="retool.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/22c6b2e3-aeda-44c6-8f54-3b7e80db129b.png">


<a id="markdown-完全理解の鍵はイメージ" name="完全理解の鍵はイメージ"></a>
### 完全理解の鍵はイメージ
Gitを扱う上で、重要なのは「何」から「何」へ、が「どんな作業」を行うのかを追うことが大事です。コマンドだけで操作していると、現状何が起きているかを理解できず、誤ったコマンドを打ってしまう可能性があります。

```
操作する前に、「何」から「何」へ、が「どんな作業」が行われるかをイメージしよう
```

<a id="markdown-新しい作業を始める" name="新しい作業を始める"></a>
## 新しい作業を始める

<a id="markdown-リポジトリ" name="リポジトリ"></a>
### リポジトリ
Gitにおけるリポジトリとは、ファイルを保存しておくための倉庫です。種類に分けると、リモートとローカルの二つがあります。リモートリポジトリは、ソースコードをインターネットや自社のサーバー上に置いて、みんなで共有できるような状態になっているリポジトリです。ローカルリポジトリは、ソースコード等を手元パソコンに置いて、自分だけが変更できる状態になっているリポジトリです。

<a id="markdown-リポジトリを複製して作業開始" name="リポジトリを複製して作業開始"></a>
### ポジトリを複製して作業開始
まず、自分の開発環境を用意します。といっても、どこでのフォルダで作業するかを決めるだけです。例えば、ホームフォルダでもいいし、普段使っているフォルダで構いません。

次に、リモートリポジトリからファイルをコピーして持ってきます。これを**clone**といいます。この段階で一緒に作業フォルダが作成されます。なので、新しいフォルダを作成しなくていいです。`git clone`というコマンドで行います。


<img width="400" alt="clone.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/36772bce-9111-1f50-bafd-97246035e78e.png">

今回はファイルが一つ(`first.txt`)だけのリポジトリをcloneしました。

<a id="markdown-ファイルを変更・追加する" name="ファイルを変更・追加する"></a>
### ファイルを変更・追加する
リポジトリのソースコードの変更は、ワーキングディレクトリ、ステージングエリアを通して行われます。実際には、我々がソースコードを変更するのは、ワーキングディレクトリです。新しく、`second.txt`というファイルを作成するとワーキングディレクトリにファイルが追加されます。

ワーキングディレクトリは、何も特殊なことはなくて、いつもパソコンで作業する時にファイルを作成する時に自分が操作しているディレクトリのことです。Gitが管理する対象のディレクトリ（今回であれば`project`）には、Gitのステージングエリアやローカルリポジトリと接続できると考えてもらえれば分かりやすいと思います。

<img width="400" alt="clone3.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/162c1db8-8271-fd4e-f702-2f77a0329564.png">


次に、ステージングエリアに変更したファイルを追加します。これを**add**と言います。変更したファイルをローカルリポジトリに反映させる前にワンクッションおくのがGitの特徴で、なぜこのクッションがあるのか後ほど詳しく説明します。`git add`というコマンドで行います。

<img width="400" alt="add.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/e975ecbf-c65f-f9ab-6dec-75984de363d6.png">

そして、ステージングエリアで追加された内容をローカルリポジトリへ登録します。これを**commit**と言います。`git commit`というコマンドで行います。ちなみに、コミットする際にコメントがかけます。今回だったら新しいファイルを追加したので、`git commit -m 'add second.txt'`とでもしておきましょう。

<img width="400" alt="commit.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/f8908fd7-72b1-51d7-4aa6-c742b56a4fac.png">

```
コミットをすると、リポジトリにコミットオブジェクトが作成されます。このときファイルのの時の状態（スナップショット）を登録していることになります。コミットオブジェクトを簡単に説明すると、更新者の情報や変更点、変更後のファイルが保存されているデータです。
Gitのオブジェクトについては、[Gitのオブジェクトの中身](https://zenn.dev/kaityo256/articles/objects_of_git)を参考にしてください。
```

<a id="markdown-リモートリポジトリを変更する" name="リモートリポジトリを変更する"></a>
### リモートリポジトリを変更する
上記作業で、自分の手元の作業は終わりました。最後に行うのが、ローカルリポジトリの変更をリモートリポジトリへ反映させることです。これを**push**と言います。リモートリポジトリに対して行うcommitだと考えると分かりやすいかもしれません。

<img width="400" alt="push.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/d57789fe-08f2-3234-ec5a-bf1457a6cb6d.png">

### 差分をみる
`diff`というコマンドを使うと作業内容を簡単に確認することができます。そもそもvscodeの拡張機能があれば特に使用することはないかもしれませんが、コミットオブジェクト同士を比較することもできるので、覚えて損はないです。
`add`する前にワーキングディレクトリで作業中の時、元々のワーキングディレクトリとの変更点をみる`git diff`、`add`した後に元のワーキングディレクトリとの変更点を見るなら`git diff --stage`、コミットを比較するなら`git diff <commit> <commit>`など。

<a id="markdown-まとめ" name="まとめ"></a>
### まとめ
一度`clone`して,作業ごとに`add`,`commit`,`push`が基本的な作業の流れです。
![basic.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/9dccfc0e-f2d9-5405-29c4-890f34894308.gif) 

```
`clone`:リモートリポジトリから自分の開発環境（ローカルリポジトリとワーキングディレクトリ）にコピーを作る。
`add`:ワーキングディレクトリからステージングエリアにファイルを追加し、コミットの準備をする。
`commit`:ステージングエリアからローカルリポジトリに登録する。この時、コミットオブジェクトが作成される。
`push`:ローカルリポジトリからリモートリポジトリへ変更内容を登録する。
```



# ブランチ
ブランチは、ファイルの変更や追加を複数の分岐で作業を行うためのものです。今まで行ってきたのは、実は`main`ブランチで作業を行なっていました。なぜブランチを作るかというと、`main`ブランチで保存しているファイルは、現在進行形で使用されているからです。なので、Gitでは、基本的に別のブランチを作って（生やして）作業を行なっていきます。

<img width="450" alt="brancha.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/76f507a4-232e-894e-e01f-328ab7138577.png">

ブランチは、

<img width="450" alt="remotes.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/9c014980-79e5-bba6-c3d8-c8f6a237178d.png">



本文・・・

### 新しいブランチを作る
`develop`というブランチを作ってみます。ブランチはリポジトリ内で管理されています。`git branch <new branch>`や`git checkout -b <new branch>`というコマンドで作ることができます。前者はブランチを作ったけどそのブランチに移動せず、後者はブランチを作ってそのブランチに移動します。

<img width="450" alt="cretae_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a7f499f9-bf5e-0c58-1a35-59cd3e2a4c30.png">

ちなみに、なぜ「生やす」とも呼ばれるかというと、どのブランチから派生させるかを決めることができるからです。派生元を`git checkout -b <new branch> <from branch>`として指定することができます。指定しなければ現在作業しているブランチが`<from branch>`になります。


:::note warn
ブランチは、実はコミットオブジェクト（厳密にいうとコミットオブジェクトのハッシュ値）のポインタです。新しブランチを生やすということは、派生元のブランチがポインタしているコミットオブジェクトを、新しいブランチもポインタすることを意味します。
:::

### ブランチで作業を進める
作業するブランチを移動することを`checkout`すると言います。現在作業しているブランチのポインタを`HEAD`と呼びます。つまり、`main`ブランチから`develop`ブランチ移動するというのは`HEAD`を変えることを意味します。

<img width="450" alt="checkout_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/4368a590-f07f-5733-e0a4-438c31eacfd5.png">

現在は、`Atr3ul`というコミットを両方のブランチが指しています。先ほどは`second.txt`を`main`ブランチでコミットして追加したので、`f27baz`というコミットからひとつ前に進んでいる状態です。ここから、`develop`ブランチで`second.txt`を変更し、新しいコミットを行うとします。

<img width="450" alt="in_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/1ae0aa22-4c8d-411b-b20e-c29047dcdb4d.png">


そうすると図のように、`develop`ブランチは`m9sgle`というコミットを作成し、そのコミットをポインタすることになりました。

現在のHEADの位置（作業ブランチの位置）やファイルがどの段階まで作業を進めたか、あるいは誰がその作業を行なっているかの状態を`status`と言います。ちなみに`git status`をすると、詳細な情報が確認できます。

:::note
コミットの矢印の向きについて、もしクラス図などオブジェクト指向の考え方に慣れているとなんとなくピンとくる方もしれません。これは「親」コミットと「子」コミットの関係を表しています。`親←-----子`、つまり親(コミット)から生まれた子(コミット)がどれだけ成長（変化）したかというのが前提としてあるわけです。
:::

### （余談）Git FlowとGitHUb Flow
ブランチの生やし方や運用は、開発チームごとによって異なると思います。一方で、プログラミングの命名規則のように、Gitのブランチの生やし方には一般的なモデルが存在します。簡単に２つを紹介します。こんなものがあるんだな程度でいいと思います。

- 「Git Flow」は、かなり複雑に入り組んだ構造をしています。本来のあるべきGitの使い方みたいなモデルかなと思います。

<img width="450" alt="git_flow.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a05bf11e-8ad0-e82f-e5f4-76498f4b5c46.png">

各ブランチの定義
`master`:プロダクトとしてリリースする用のブランチ。※このブランチ上での作業は行わない
`develop`:開発用ブランチ。コードが安定し、リリース準備ができたらreleaseへマージする。※このブランチ上での作業は行わない
`feature`:機能の追加用。developから分岐し、developにマージする。
`hotfix`:リリース後の緊急対応（クリティカルなバグフィックスなど）用。masterから分岐し、masterにマージすると共にdevelopにマージする。
`release`:プロダクトリリースの準備用。リリース予定の機能やバグフィックスが反映された状態のdevelopから分岐する。リリース準備が整ったら、masterにマージすると共にdevelopにマージする。

- 「GitHUb Flow」は、Git Flowをやや簡略化したモデルです。

<img width="450" alt="github_flow.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/b717c9a0-7612-906f-2f10-3bf7a600e755.png">

見ての通り、`main`と`feature`だけで構成されており、主な違いとしてプルリクエスト（下のプルで説明）というクッションでブランチ間の統合を行います。

### まとめ
基本的にmain上で作業することはないので、行いたい作業単位でブランチを作成し、新しいコミットを作成していきましょう。

![branch_anime.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/f5158fea-ea6c-a033-11f7-27a9b152539c.gif)

:::note
`branch`:コミットに対する新しいポインタ
`checkout`:`HEAD`を移動させて、作業する`branch`を変える。
:::

# マージ
枝分かれたブランチ同士を統合することでを`merge`すると言います。基本的に、`main`ブランチや`develop`ブランチに対して統合を行なっていきます。注意点は、「どのブランチ」が「どのブランチ」を統合(吸収)するかの主語を間違わないことです。必ず、派生元のブランチに（HEAD）移動して、派生先のブランチからの統合を行うことになります。

現在、`feature`ブランチで作業を行なっていて、下記のような　`third.txt`を作成しました。　`add`して`commit`まで終えました。


```third.txt
Hello, World! I'm noshishi, from Japan.
I like dancing on house music.
```
<img width="450" alt="feature_commit.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/3fbb3116-4439-7753-74ec-260b043777f3.png">


### ファストフォワードでマージする
`feature`ブランチが、派生元である`develop`ブランチから辿れるのコミットを指しているとき、`develop`ブランチは`fast-forward`な状態と言います。

まずは、　`checkout`で`develop`へ移動します。

<img width="450" alt="checkout_develop.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/d2564a3b-ab4f-32a3-edb4-5dffb38c7f89.png">

この場合、`develop`ブランチは全く進んでいないので、`feature`ブランチを`merge`すると、単にコミットを前に進めるだけになります。この時、`develop`ブランチと`feature`ブランチは同じコミットを共有することになります。

<img width="450" alt="merge_feature_no_conflict.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/95d4195d-a9a5-1fec-710e-85887c2f7f1f.png">

### ノーファストフォワード
もし、`develop`ブランチが違うコミットやマージによって、全くことなるコミットに進んでいたらどうなるでしょうか？これを`no fast-forward`な状態と言います。

`develop`ブランチでは、`first.txt`を変更を行なって`commit`まで終えました。そのため、`develop`ブランチと'`feature`ブランチは、完全に枝分かれてしまいました。

<img width="450" alt="develop_commi.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/b632debb-64ae-8421-4a91-77bf3893ff20.png">

この時、`develop`ブランチから、`feature`ブランチを`merge`しようとすると、Gitは変更履歴同士を確認します。もしお互いに競合しあう編集をしていない場合は、すぐに`merge commit`が作成されます。

<img width="450" alt="merge_feature_auto.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a81e9577-6aec-e565-71bf-0836fd883974.png">

### コンフリクトに対処する

`no fast-forward`状態で、作業内容が競合していることを`conflict`と言います。この場合は、手動でコンフリクトをしている内容を修正し、コミットを行います。

`develop`ブランチでは、以下のような`third.txt`が作成され、`commit`されています。

```third.txt
Hello, World! I'm nope, from USA.
I like dancing on house music.
```

`develop`ブランチでは、`I'm nope, from USA`となっており、`feature`ブランチでは、`I'm noshishi, from Japan`となっており、1行目の内容が競合している状態です。この時に`merge`を行うと、Gitが`conflict`が起こっているので、それを解決してから`commit`してくれと指示が出てきます。

<img width="450" alt="conflict.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/0732c833-6e19-9255-66f5-2e80c33cc921.png">

（作業場所は`develop`ブランチ）指示通りに`third.txt`をみてみると、以下のような追記がされています。

```third.txt　（conflict後）
<<<<<<< HEAD
Hello, World! I'm noshishi, from Japan.
=======
Hello, World! I'm nope, from USA.
>>>>>>> feature
I like dancing on house music.
```

`=======`で区切られた上側の`HEAD`が`develop`ブランチの内容を表しており、下側が`feature`ブランチを表しています。まずどちらを採用するかを考え、今回は`feature`ブランチでの変更内容を採用することにしました。その時の作業は、手作業で`third.txt`を編集（不要な部分を削除）するだけです。

```third.txt　(編集後)
Hello, World! I'm noshishi, from Japan.
I like dancing on house music.
```

そして次に行うのが、`add`して`commit`です。`conflict`が解消され、新しい`merge commit`が作成されます。

<img width="450" alt="hand_merge.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/5a20b54e-33bd-a973-e201-8bbdaba8c0cc.png">

初心者が恐怖のコンフリクトですが、これを覚えればもう怖くありません。

:::note warn
`merge`を行なって`conflict`を解消したらなぜもう一度`merge`しないのでしょうか？と疑問に思うと思います。`merge`を一度実行した時点で`develop`ブランチは`merge`状態に入り、ブランチ同士のコミット中身で新しいファイルを自動で`add`,`commit`を行なってくれています。なので、`conflict`を解消した後に。**特別に`commit`しているのではない**ということです。だからこそ`merge commit`と呼ばれるわけです。
:::


### 不要なブランチは削除する
統合されたブランチは、基本お役御免なので、削除していきます。ブランチを放置しておくと、削除したいブランチから他のブランチに移動して、`git branch -d <branch>`すればおさらばです。ちなみに、削除されたらそのブランチのコミットオブジェクトは無くなるのかというとそうではありません。マージしたブランチにしっかりと引き継がれています。`git log`を使用すると、過去のすべてのコミットを閲覧できます。

### (余談)ブランチの正体を知りたい
ブランチは、コミットオブジェクトを指すポインタと言いましたが、もうひとつ重要なデータを保持しています。それは、そのブランチで行ってきたすべてのコミットオブジェクトです。つまり、ブランチは、コミットオブジェクトの集合体であり、なおかつその中で最新のコミットオブジェクトを指すポインタを持っているということです。図で表すと以下の通りです。

<img width="450" alt="branch_image.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/8cbb8c69-d048-778c-9d2d-db1f3be3b3be.png">

だから、Git Flowの様に横軸でブランチを考えることができるんです。ちなみに、上の図を横軸にブランチを置いて書いてみるとこうなります。

<img width="450" alt="branch_image2.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/97ed7459-a4ee-afd9-1e2e-d7a0ffbf8f1e.png">

### まとめ
`fast-forward merge`

![fast_merge.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/4abed1f6-7431-50d5-b90a-58e152633096.gif)

`no fast-forward merge`

![nofast_merge1.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/3a29143b-fe53-3df7-352c-ccd569373685.gif)

`no fast-forward merge with conflict`

![nofast_merge2.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/2898da3a-dc15-e1fd-c991-61d80ca0f6a1.gif)


:::note
`merge`:特定のブランチ(`main`や`develop`など)に、作業用のブランチ（`feature`など）を統合（吸収）し、新しいコミットを作成すること。
:::

# リベース
各ブランチの派生元のコミットを変えてブランチ同士を統合することを`rebase`と言います。`merge`と非常に似ていますが、大きく違うところは作業を行うブランチが 「派生先」のブランチだということです。`develop`ブランチと`feature`ブランチで作業していているとしましょう。

<img width="450" alt="base_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/0655a75b-4466-a069-0ad7-f98a2785996c.png">

### ブランチごとごっそり移動させる
`develop`ブランチの現在のコミット、`feature`ブランチに反映させるためには、`feature`ブランチが派生した`----`から`---`に移動させる必要があります。これは`feature`ブランチから`git rebase develop`とすることで一気に移動できます。

<img width="450" alt="move_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/acecd79a-66eb-1836-f911-a76b2ad3dbef.png">

マージをしているというより、`develop`ブランチの最新のコミットから`feature`ブランチを生やし直すに近いです。ただし、コミットしたオブジェクトごと移動している点が大きな違いです。

<img width="450" alt="rebase_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/d37b85ab-7b78-8a68-3302-6081edc036b1.png">

なぜ、こんな移動（統合）をするのかというと、コミットが一直線になることで、コミットの歴史あ

### リベースのコンフリクトに対処する
もちろん`rebase`にも`conflict`が存在します。上記の場合、`feature`ブランチでは、`fourth.txt`を追加しましたが、その後`develop`ブランチでのコミットでは、`fourth.txt`に関わる変更がないため、`conflict`は起こりません。ですが、以下のように

<img width="450" alt="rebase_conflict.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/c04e5c00-7523-ccfd-9311-421fe252fdc0.png">


# ローカルリポジトリを最新にする
### 最新の状況を確認する
[図の挿入]

### 最新の状態に更新する
プル = フェッチ + マージ

[図の挿入]

### プルのコンフリクトに対処する

### （余談）プルリクエストの正体

[図の挿入]



# 便利な機能
### リバート
「リバート(revert)」は
[図の挿入]

### リセット
「リセット（reset）」
[図の挿入]

### スタッシュ


### チェリーピッキング

別のブランチのコミットをとってくる
[図の挿入]

# 終わりに
### Git以外のソースコードの管理
Gitと同じ歴史を持ったMercurialというサービスがあります。特徴は、Gitのような柔軟性を犠牲に非常にシンプルなコマンドラインインターフェース(CLI)を採用していることです。最近だと、このMercurialをベースにMeta社がSaplingという新しいソースコード管理システムをオープンソースで公開されましたね。また今度、ちょっと触ってみて感想を書いてみたいなと思います。

### リモートリポジトリの居場所
リモートリポジトリ用のサーバーを貸してくれるサービスをホスティングサービスと言います。代表的なものであれば、GitHub, Bitbucket、プライベートに使用するAws Code Commitなどがあります。GitとGit Hubは、全く別物です。ちなみに、上で書いた通り、リモートリポジトリ用のサーバーは自分達のサーバーでも大丈夫です。

### さらにGitを理解するために
この記事で言及できなかったことがたくさんあります。Gitのコアの部分はシンプルなキー・バリュー型データストアであることや、バリューとなるGitオブジェクトの詳細、そしてオブジェクトをどのように扱うかなど。いつか完全攻略したいと思います。


### お礼
最後まで読んでいただきありがとうございました。この記事の作成を通して、本当の意味でGitと向き合えました。まだまだ見習いエンジニアなので、なんとか小手先で解決したくなるGitのコマンドでしたが、最近ではイメージが湧いてくるのでコマンドが楽しくなってきました。誰かにとって同じような体験となれば幸いです。[GitHub](https://github.com/nopeNoshishi/learn_git)でも公開しています。GitHub上ではコマンドあり版も記載していく予定なので、もしよければご覧ください。

（※公開と同時に非公開から公開にする予定です）

# 参考サイト
- [Git Documentation](https://git-scm.com/doc)
- [Learn git concepts, not commands](https://dev.to/unseenwizzard/learn-git-concepts-not-commands-4gjc)
- [図解 Git](https://marklodato.github.io/visual-git-guide/index-ja.html)
- [いまさらだけどGitを基本から分かりやすくまとめてみた](https://qiita.com/gold-kou/items/7f6a3b46e2781b0dd4a0)
- [git add ってなんのためにやるの？](https://kray.jp/blog/expound-git-add/)
