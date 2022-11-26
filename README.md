# コマンドを使わずに理解するGit

<!-- TOC -->

- [Gitとは](#Gitとは)
	- [バージョンを管理し、作業を分散する](#バージョンを管理し、作業を分散する)
	- [Gitを使うということ](#Gitを使うということ)
	- [完全理解の鍵はイメージ](#完全理解の鍵はイメージ)
- [新しい作業を始める](#新しい作業を始める)
	- [リポジトリ](#リポジトリ)
	- [リポジトリを複製して作業開始](#リポジトリを複製して作業開始)
	- [(補足)ワーキングディレクトリ](#ワーキングディレクトリ)
	- [ファイルを変更・追加する](#ファイルを変更・追加する)
	- [リモートリポジトリを変更する](#リモートリポジトリを変更する)
	- [差分をみる](#差分をみる)
	- [(余談)ステージングエリアというクッション](#ステージングエリアというクッション)
	- [まとめ](#まとめ)
- [ブランチ](#ブランチ)
	- [新しいブランチを作る](#新しいブランチを作る)
	- [ブランチで作業を進める](#ブランチで作業を進める)
	- [(余談)Git-FlowとGitHub-Flow](#Git-FlowとGitHub-Flow)
	- [まとめ](#まとめ)
- [マージ](#マージ)
	- [ファストフォワード](#ファストフォワード)
	- [ノーファストフォワード](#ノーファストフォワード)
	- [コンフリクトに対処する](#コンフリクトに対処する)
	- [不要なブランチは削除する](#不要なブランチは削除する)
	- [(余談)ブランチの正体を知りたい](#ブランチの正体を知りたい)
	- [まとめ](#まとめ)
- [リベース](#リベース)
	- [ブランチごとごっそり移動させる](#ブランチごとごっそり移動させる)
	- [リベースのコンフリクトに対処する](#リベースのコンフリクトに対処する)
- [ローカルリポジトリを最新にする](#ローカルリポジトリを最新にする)
	- [ブランチとリポジトリ](#ブランチとリポジトリ)
	- [最新の状況を確認する](#最新の状況を確認する)
	- [最新の状態に更新する](#最新の状態に更新する)
	- [プルのコンフリクトに対処する](#プルのコンフリクトに対処する)
	- [（余談）プルリクエストの正体](#プルリクエストの正体)
- [便利な機能](#便利な機能)
	- [リバート](#リバート)
	- [スタッシュ](#スタッシュ)
	- [チェリーピック](#チェリーピック)
	- [HEADを使いこなす](#HEADを使いこなす)
	- [\(おまけ\)リセット](#リセット)
- [終わりに](#終わりに)
	- [Git以外のソースコードの管理](#Git以外のソースコードの管理)
	- [リモートリポジトリの居場所](#リモートリポジトリの居場所)
	- [ポインタ](#ポインタ)
	- [さらにGitを理解するために](#さらにGitを理解するために)
	- [お礼](#お礼)
- [参考サイト](#参考サイト)

<!-- TOC -->

<a id="markdown-Gitとは" name="Gitとは"></a>
## Gitとは

<a id="markdown-バージョンを管理し、作業を分散する" name="バージョンを管理し、作業を分散する"></a>
### バージョンを管理し、作業を分散する
Gitは、分散型バージョン管理システムと呼ばれるソースコードの管理システムの1種です。
Gitは、ファイルの**変更履歴（バージョン）を記録・追跡**することで、過去と現在のファイルを比較し、変更点を明らかにすることで、円滑に開発作業を進めるためのツールです。
また、一度に**複数の開発者がファイルを編集**できるシステムなので、作業を分散して行うことができます。

<a id="markdown-Gitを使うということ" name="Gitを使うということ"></a>
### Gitを使うということ
まず、みんなで共有できる保存場所（以下、リモートリポジトリ）にあるファイルなどを、手元のパソコン（以下、ローカルリポジトリ）にコピーを作って、新しいコードやファイルを追加・編集します。
そして、ローカルリポジトリからリモートリポジトリへ登録することでファイルを更新していくことです。

<img width="600" alt="retool.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/22c6b2e3-aeda-44c6-8f54-3b7e80db129b.png">


<a id="markdown-完全理解の鍵はイメージ" name="完全理解の鍵はイメージ"></a>
### 完全理解の鍵はイメージ
Gitを扱う上で、重要なのは「何」から「何」へ、が「どんな作業」を行うのかを追うことです。
コマンド操作だけだと、何が起きているかを理解できず、誤ったコマンドを入力する可能性があります。


**(info)**

Gitの操作は、操作前と操作後でどんなことが起こっているのかをイメージしよう。


<a id="markdown-新しい作業を始める" name="新しい作業を始める"></a>
## 新しい作業を始める

<a id="markdown-リポジトリ" name="リポジトリ"></a>
### リポジトリ
Gitにおけるリポジトリとは、ファイルを保存しておくための倉庫で、リモートとローカルの二つがあります。

**リモートリポジトリ**は、ソースコードをインターネットのサーバーに置いて、みんなで共有できるリポジトリです。
**ローカルリポジトリ**は、ソースコードを手元パソコンに置いて、自分だけが変更できるリポジトリです。


<a id="markdown-リポジトリを複製して作業開始" name="リポジトリを複製して作業開始"></a>
### リポジトリを複製して作業開始
まず、自分の開発環境を用意します。
といっても、どのディレクトリで作業するかを決めるだけです。
例えば、ホームディレクトリでもいいし、普段使っているディレクトリで構いません。

次に、リモートリポジトリからファイルをコピーして持ってきます。
これを`clone`といいます。

この段階で一緒に作業ディレクトリが作成されます。
なので、新しいディレクトリを作成しなくていいです。
`git clone`というコマンドで行います。


<img width="450" alt="clone.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/36772bce-9111-1f50-bafd-97246035e78e.png">

`project`というリモートリポジトリには、`first.txt`だけ入っていて、そのリモートリポジトリを`clone`した時のイメージです。

<a id="markdown-ワーキングディレクトリ" name="ワーキングディレクトリ"></a>
### (補足)ワーキングディレクトリ
ワーキングディレクトリは、何も特殊なディレクトリではなくて、いつもパソコンで作業するディレクトリのことです。
Gitが管理する対象のディレクトリ（今回であれば`project`）には、Gitのステージングエリアやローカルリポジトリと接続できると考えてもらえれば分かりやすいと思います。

<img width="400" alt="clone3.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/162c1db8-8271-fd4e-f702-2f77a0329564.png">

<a id="markdown-ファイルを変更・追加する" name="ファイルを変更・追加する"></a>
### ファイルを変更・追加する
リポジトリのソースコードの変更は、ワーキングディレクトリ、ステージングエリアを通して行われます。
実際には、我々がソースコードを変更するのは、ワーキングディレクトリです。

新しく、`second.txt`というファイルを作成してみます。

<img width="450" alt="create_file.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/8361cfea-fca9-3193-835b-e31b449df6b8.png">

次に、ステージングエリアに変更したファイルを追加します。
これを`add`と言います。

変更したファイルをローカルリポジトリに反映させる前にワンクッションおくのがGitの特徴です。
なぜこのクッションがあるのか後ほど詳しく説明します。
`git add`というコマンドで行います。

<img width="450" alt="add.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/e975ecbf-c65f-f9ab-6dec-75984de363d6.png">

そして、ステージングエリアで追加された内容をローカルリポジトリへ登録します。
これを`commit`と言います。
`commit`するとステージングエリアは、空になります。

ちなみに、`commit`する際にコメントがかけます。
今回なら、ファイルを追加したので、`git commit -m 'add second.txt'`と書きましょう。

<img width="450" alt="commit.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/f8908fd7-72b1-51d7-4aa6-c742b56a4fac.png">


**(info)**

コミットをすると、リポジトリに**コミットオブジェクト**が作成されます。
コミットオブジェクトを簡単に説明すると、更新者の情報や変更後のファイルが保存されているデータです。
（このときデータは、差分だけではなく、その瞬間のファイルの状態（スナップショット）全てが保存されます。）
Gitのオブジェクトについては、[Gitのオブジェクトの中身](https://zenn.dev/kaityo256/articles/objects_of_git)を参考にしてください。


<a id="markdown-リモートリポジトリを変更する" name="リモートリポジトリを変更する"></a>
### リモートリポジトリを変更する
上記作業で、自分の手元の作業は終わりました。
最後に行うのが、ローカルリポジトリの変更をリモートリポジトリへ反映させることです。
これを`push`と言います。

<img width="400" alt="push.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/d57789fe-08f2-3234-ec5a-bf1457a6cb6d.png">

リモートリポジトリに対して行うcommitだと考えると分かりやすいかもしれません。

<a id="markdown-差分をみる" name="差分をみる"></a>
### 差分をみる
同じファイルの同士の変更点を`diff`といいます。
作業の途中で自分が行った変更を確認することができます。
`git diff`というコマンドを使用します。

コマンドの詳しい説明は省きますが、よく使う３つを紹介します。
`add`する前に、元のワーキングディレクトリとの変更点をみる`git diff`。
`add`した後に、作業中のワーキングディレクトリとの変更点を見るなら`git diff --stage`。
コミット同士を比較するなら`git diff <commit> <commit>`など。

<a id="markdown-ステージングエリアというクッション" name="ステージングエリアというクッション"></a>
### (余談)ステージングエリアというクッション
開発作業が大きくなると、多くの変更を一つのワーキングディレクトリで行うことあります。
全ての変更を一気にローカルレポジトリに登録するとどうなるでしょうか？
この場合、コミットを解析する時に、ある機能をどこで実装したかわからないといったことが起こりうります。

Gitでは、１つの機能につき一つの`commit`を行うことが推奨されています。
そのために、`commit`を行う単位を細かく分けることができるステージングエリアがあるということです。

<img width="400" alt="push.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/5b25c70d-baf5-0913-d220-f60aedf583ea.png">


必要な分だけステージングし、作業を進めたり、先に`commit`することで、実装ごとに履歴を辿れる効率的な開発を進めていこうというのがGitのコンセプトなのです。

<a id="markdown-まとめ" name="まとめ"></a>
### まとめ
一度`clone`して,作業ごとに`add`,`commit`,`push`が基本的な作業の流れです。

![basic.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/9dccfc0e-f2d9-5405-29c4-890f34894308.gif) 


**(info)**

`clone`:リモートリポジトリから自分の開発環境（ローカルリポジトリとワーキングディレクトリ）にコピーを作る。
`add`:ワーキングディレクトリからステージングエリアにファイルを追加し、コミットの準備をする。
`commit`:ステージングエリアからローカルリポジトリに登録する。この時、コミットオブジェクトが作成される。
`push`:ローカルリポジトリからリモートリポジトリへ変更内容を登録する。



<a id="markdown-ブランチ" name="ブランチ"></a>
## ブランチ
ファイルの変更や追加を複数の分岐で作業を行うために`branch`を作ります。
`main`ブランチで保存しているファイルは、現在進行形で使用されています。
ブランチを分ける理由は、**現在稼働しているソースコードに影響を与えることなく作業を行う**ためです。

<a id="markdown-新しいブランチを作る" name="新しいブランチを作る"></a>
### 新しいブランチを作る
`develop`というブランチを作ってみます。
`git branch <new branch>`や`git checkout -b <new branch>`で作ることができます。
前者はブランチを作るだけ、後者はブランチを作ってそのブランチに移動します。
（ブランチはリポジトリ内で管理されています。）


<img width="450" alt="cretae_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a7f499f9-bf5e-0c58-1a35-59cd3e2a4c30.png">

ブランチを生やす時のポイントは、**どのブランチを派生元にするか**ということです。
派生元を`git checkout -b <new branch> <from branch>`として指定することができます。
指定しなければ、現在作業しているブランチが`<from branch>`になります。



**(info)**

ブランチは、実は**コミット（厳密にいうとコミットオブジェクトのハッシュ値）のポインタ**です。
新しいブランチを生やすということは、派生元のブランチがポインタしているコミットを、新しいブランチも同様にポインタすることを意味します。


<a id="markdown-ブランチで作業を進める" name="ブランチで作業を進める"></a>
### ブランチで作業を進める
作業するブランチを移動することを`checkout`すると言います。
現在作業しているブランチのポインタを`HEAD`と呼びます。
つまり、`main`ブランチから`develop`ブランチ移動するというのは`HEAD`を変えることを意味します。

<img width="450" alt="checkout_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/3a7379b1-7a1c-35ba-02be-619d13a192ad.png">


現在は、`Atr3ul`というコミットを両方のブランチが指しています。
先ほどは`second.txt`を`main`ブランチでコミットして追加したので、`f27baz`というコミットからひとつ前に進んでいる状態です。
ここから、`develop`ブランチで`second.txt`を変更し、新しいコミットを行うとします。

<img width="450" alt="in_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/1ae0aa22-4c8d-411b-b20e-c29047dcdb4d.png">


そうすると図のように、`develop`ブランチは`m9sgle`というコミットを作成し、そのコミットをポインタすることになりました。

現在のHEADの位置（作業ブランチの位置）やファイルがどの段階まで作業を進めたか、あるいは誰がその作業を行なっているかの状態を`status`と言います。
`git status`を入力すると、詳細な情報が確認できます。


**(info)**

コミットの矢印の理由について、オブジェクト指向の考え方に慣れている方だと分かるかもしれません。
これは「親」コミットと「子」コミットの関係を表しています。
`親←-子`、つまり親(コミット)から生まれた子(コミット)がどれだけ成長（変化）したかというのが、前提としてあります。


<a id="markdown-Git-FlowとGitHub-Flow" name="Git-FlowとGitHub-Flow"></a>
### （余談）Git-FlowとGitHub-Flow
ブランチの生やし方や運用は、開発チームごとによって異なると思います。
一方で、プログラミングの命名規則のように、Gitのブランチの生やし方には一般的なモデルが存在します。
簡単に２つを紹介します。こんなものがあるんだな程度でいいと思います。

<br>
<br>

「Git Flow」は、かなり複雑に入り組んだ構造をしています。
本来のあるべきGitの使い方みたいなモデルかなと思います。


<img width="450" alt="git_flow.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a05bf11e-8ad0-e82f-e5f4-76498f4b5c46.png">


**各ブランチの定義**

`master`:プロダクトとしてリリースする用のブランチ。※このブランチ上での作業は行わない

`develop`:開発用ブランチ。リリース準備ができたらreleaseへマージする。※このブランチ上での作業は行わない

`feature`:機能の追加用。developから分岐し、developにマージする。

`hotfix`:リリース後の緊急対応（クリティカルなバグフィックスなど）用。masterから分岐し、masterにマージすると共にdevelopにマージする。

`release`:プロダクトリリースの準備用。リリース予定の機能やバグフィックスが反映された状態のdevelopから分岐する。
リリース準備が整ったら、masterにマージすると共にdevelopにマージする。

<br>
<br>
「GitHub Flow」は、Git Flowをやや簡略化したモデルです。

<img width="450" alt="github_flow.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/b717c9a0-7612-906f-2f10-3bf7a600e755.png">

見ての通り、`master`と`feature`だけで構成されており、主な違いとしてプルリクエスト（下のプルで説明）というクッションでブランチ間の統合を行います。

<a id="markdown-まとめ" name="まとめ"></a>
### まとめ
基本的にmain(master)上で作業することはないので、行いたい作業単位でブランチを作成し、新しいコミットを作成していきましょう。

![branch_anime.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/f5158fea-ea6c-a033-11f7-27a9b152539c.gif)

**(info)**

`branch`:コミットに対する新しいポインタ
`checkout`:`HEAD`を移動させて、作業する`branch`を変える。


<a id="マージ-まとめ" name="マージ"></a>
## マージ
枝分かれたブランチ同士を統合することを`merge`と言います。
基本的に、`main`ブランチや`develop`ブランチに対して統合を行なっていきます。

注意点は、**「どのブランチ」が「どのブランチ」を統合(吸収)するかの主語を間違わないこと**です。
必ず、派生元のブランチに（HEADを）移動して、派生先のブランチからの統合を行うことになります。

現在、`feature`ブランチで作業を行なっていて、下記のような　`third.txt`を作成しました。

```text third.txt
Hello, World! I'm noshishi, from Japan.
I like dancing on house music.
```

<br>
そして、`add`して`commit`まで終えました。  
<br>

<img width="450" alt="feature_commit.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/3fbb3116-4439-7753-74ec-260b043777f3.png">


<a id="markdown-ファストフォワード" name="ファストフォワード"></a>
### ファストフォワード
`feature`ブランチが、派生元である`develop`ブランチから辿れるのコミットを指しているとき、`develop`ブランチは`fast-forward`な状態と言います。

まずは、`checkout`で`develop`へ移動します。

<img width="450" alt="checkout_develop.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/d2564a3b-ab4f-32a3-edb4-5dffb38c7f89.png">

この場合、`develop`ブランチは全く進んでいないので、`feature`ブランチを`merge`すると、単にコミットを前に進めるだけになります。
この時、`develop`ブランチと`feature`ブランチは同じコミットを共有することになります。

<img width="450" alt="merge_feature_no_conflict.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/95d4195d-a9a5-1fec-710e-85887c2f7f1f.png">

<a id="markdown-ノーファストフォワード" name="ノーファストフォワード"></a>
### ノーファストフォワード
もし、`develop`ブランチがコミットやマージによって、新しいコミットに進んでいたらどうなるでしょうか？
これを`no fast-forward`な状態と言います。

`develop`ブランチでは、`first.txt`を変更を行なって`commit`まで終えました。
そのため、`develop`ブランチと'`feature`ブランチは、完全に枝分かれてしまいました。

<img width="450" alt="develop_commi.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/d97a8652-dd94-2835-4cf2-94eed01e6353.png">


`develop`ブランチから、`feature`ブランチを`merge`しようとすると、Gitは変更履歴同士を確認します。
もしお互いに競合しあう編集をしていない場合は、すぐに`merge commit`が作成されます。
これを`Automatic merge`と呼ばれます。

<img width="450" alt="merge_feature_auto.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a81e9577-6aec-e565-71bf-0836fd883974.png">

<a id="markdown-コンフリクトに対処する" name="コンフリクトに対処する"></a>
### コンフリクトに対処する

`no fast-forward`状態で、作業内容が競合していることを`conflict`と言います。
この場合は、手動で`conflict`をしている内容を修正し、`commit`を行います。


`develop`ブランチでは、以下のような`third.txt`が作成され、`commit`されています。


```text third.txt
Hello, World! I'm nope, from USA.
I like dancing on house music.
```
<br>

`develop`ブランチでは、`I'm nope, from USA`と書いてあり、
`feature`ブランチでは、`I'm noshishi, from Japan`と書いてあります。
1行目の内容が競合している状態です。

この時に`merge`を行うと、`conflict`が起こります。
Gitが`conflict`を解決してから`commit`してくれと指示が出てきます。

<img width="450" alt="conflict.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/794285e4-077a-17b1-4743-0e1f2685fba7.png">


<br>
（作業場所は`develop`ブランチ）

指示通りに`third.txt`を見てみると、以下のような追記がされています。

```text third.txt　（conflict後）
<<<<<<< HEAD
Hello, World! I'm noshishi, from Japan.
=======
Hello, World! I'm nope, from USA.
>>>>>>> feature
I like dancing on house music.
```

`=======`で区切られた上側の`HEAD`が`develop`ブランチの内容を表しています。
下側が`feature`ブランチを表しています。

まずどちらを採用するかを考え、今回は`feature`ブランチでの変更内容を採用することにしました。
その時の作業は、`third.txt`を手作業で編集（不要な部分を削除）するだけです。

```text third.txt　(編集後)
Hello, World! I'm noshishi, from Japan.
I like dancing on house music.
```

そして次に行うのが、`add`して`commit`です。
`conflict`が解消され、新しい`merge commit`が作成されます。

<img width="450" alt="hand_merge.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/5a20b54e-33bd-a973-e201-8bbdaba8c0cc.png">

初心者が恐怖のコンフリクトですが、これを覚えればもう怖くありません。


**(info)**

`merge`を行なって`conflict`を解消したらなぜもう一度`merge`しないのでしょうか？と疑問に思うと思います。
`merge`を一度実行した時点で`develop`ブランチは`merge`状態に入り、ブランチ同士の比較し`conflict`がなければ、新しいファイルを自動で`add`,`commit`を行なってくれています。
なので、`conflict`を解消した後に、**特別に`commit`しているのではない**ということです。
だからこそ`merge commit`と呼ばれるわけです。


<a id="markdown-不要なブランチは削除する" name="不要なブランチは削除する"></a>
### 不要なブランチは削除する
統合されたブランチは、基本お役御免なので、削除していきます。
ブランチを放置しておくと、削除したいブランチから他のブランチに移動して、`git branch -d <branch>`すればおさらばです。
ちなみに、削除されたらそのブランチのコミットは無くなるのかというとそうではありません。
マージしたブランチにしっかりと引き継がれています。
`git log`を使用すると、ブランチ内で行なったすべてのコミットおよびマージしたブランチのコミットを閲覧できます。

<a id="markdown-ブランチの正体を知りたい" name="ブランチの正体を知りたい"></a>
### (余談)ブランチの正体を知りたい
ブランチは、コミットを指すポインタと言いましたが、もうひとつ重要なデータを保持しています。
それは、そのブランチで行ってきたすべてのコミットです。
つまり、ブランチは、コミットの集合体であり、なおかつその中で最新のコミットを指すポインタを持っているということです。

図で表すと以下の通りです。

<img width="450" alt="branch_image.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/8cbb8c69-d048-778c-9d2d-db1f3be3b3be.png">

だから、Git Flowの様に横軸でブランチを考えることができるんです。
ちなみに、上の図を横軸にブランチを置いて書いてみるとこうなります。

<img width="450" alt="branch_image2.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/97ed7459-a4ee-afd9-1e2e-d7a0ffbf8f1e.png">

<a id="markdown-まとめ" name="まとめ"></a>
### まとめ
`fast-forward merge`
![fast_merge.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/4abed1f6-7431-50d5-b90a-58e152633096.gif)

`no fast-forward merge`
![nofast_merge1.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/3a29143b-fe53-3df7-352c-ccd569373685.gif)

`no fast-forward merge with conflict`
![nofast_merge2.gif](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/2898da3a-dc15-e1fd-c991-61d80ca0f6a1.gif)


**(info)**

`merge`:特定のブランチ(`main`や`develop`など)に、作業用のブランチ（`feature`など）を統合（吸収）し、新しいコミットを作成すること。


<a id="markdown-リベース" name="リベース"></a>
## リベース
各ブランチの派生元のコミットを変えてブランチ同士を統合することを`rebase`と言います。
`merge`に似ていますが、異なる点は、作業を行うブランチが 「派生先」ブランチということです。

`develop`ブランチと`feature`ブランチで作業していているとしましょう。

<img width="450" alt="base_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/0655a75b-4466-a069-0ad7-f98a2785996c.png">

<a id="markdown-ブランチごとごっそり移動させる" name="ブランチごとごっそり移動させる"></a>
### ブランチごとごっそり移動させる
`develop`ブランチの現在のコミット、`feature`ブランチに反映させるためには、`feature`ブランチが派生した`gp55sw`コミットから`3x7oit`コミットに移動させる必要があります。

これは`feature`ブランチから`git rebase develop`とすることで一気に移動できます。

<img width="450" alt="move_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/acecd79a-66eb-1836-f911-a76b2ad3dbef.png">


`merge`をしているというより、`develop`ブランチの最新のコミットから`feature`ブランチを生やし直すに近いです。
ただし、コミットごと移動し、新しいコミットを行うことが違いです。

<img width="450" alt="rebase_branch.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/d37b85ab-7b78-8a68-3302-6081edc036b1.png">

なぜ、こんな移動（統合）をするのかというと、一つは　`fast-forward`になりいつでも`merge`を行いやすいことです。
もう一つは、コミットが一直線になることで容易にコミット履歴を辿れ、ファイルの更新順に整合性を持たせることができるためです。

<a id="markdown-リベースのコンフリクトに対処する" name="リベースのコンフリクトに対処する"></a>
### リベースのコンフリクトに対処する
もちろん`rebase`にも`conflict`が存在します。
上記の場合、`feature`ブランチでは、`fourth.txt`を追加しましたが、その後`develop`ブランチでのコミットでは、`fourth.txt`に関わる変更がないため、`conflict`は起こりません。

ですが、以下のように変更内容が被っていた場合、`conflict`が起こります。

<img width="450" alt="rebase_conflict.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/c04e5c00-7523-ccfd-9311-421fe252fdc0.png">

でも`merge`と同じように対処すれば大丈夫です。
ただし、差分を確認してファイルの編集を終えたら、`git rebase --continue`で作業を終わりましょう。
`commit`しなくても自動でコミットしてくれます。


**(info)**

`rebase`:ブランチの派生元であるコミットを移動させて新しいコミットを行うこと。


<a id="markdown-ローカルリポジトリを最新にする" name="ローカルリポジトリを最新にする"></a>
## ローカルリポジトリを最新にする
ローカルである程度作業を進めると、リモートリポジトリが他の開発者によって更新されている場合があります。
この場合において、リモートリポジトリの情報を再度ローカルリポジトリに反映させるために行うのが`pull`です。

<a id="markdown-ブランチとリポジトリ" name="ブランチとリポジトリ"></a>
### ブランチとリポジトリ
ブランチは、各リポジトリに保存されています。
実際に作業を行うブランチです。

<img width="450" alt="brancha.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/76f507a4-232e-894e-e01f-328ab7138577.png">

一方で、ローカルリポジトリには、リモートリポジトリをコピーしたブランチがあります。
これは「リモート追跡ブランチ」と呼ばれます。
`remotes/<remote branch>`でリモートのブランチと紐づく名前のブランチです。

これは、あくまでもリモートリポジトリを監視しているに過ぎません。

<img width="450" alt="remotes.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/9c014980-79e5-bba6-c3d8-c8f6a237178d.png">


<a id="markdown-最新の状況を確認する" name="最新の状況を確認する"></a>
### 最新の状況を確認する
リモートリポジトリの`develop`ブランチがリモート追跡ブランチより一つ進んでいる状況だったとします。。

<img width="450" alt="pull_notupdate.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/3393d6c6-1a65-31c7-7afb-f98170f79872.png">

リモートリポジトリのブランチの最新の状況をリモート追跡ブランチに反映させることを`fetch`と言います。

<img width="450" alt="fetch_update.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/ff486c7c-963d-5962-0878-a144aab5893f.png">


<a id="markdown-最新の状態に更新する" name="最新の状態に更新する"></a>
### 最新の状態に更新する
もう少し踏み込んで、ローカルブランチにも反映させたい場合、`pull`を行います。
`pull`するとまず、ローカルのリモート追跡ブランチが更新されます。
その後にローカルブランチに`merge`を行います。

<img width="450" alt="pull_update.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a9526959-34d0-20fd-d128-4b5b37f19298.png">


今回は、`develop`ブランチの一つ先に進んだコミットがあったので、ローカルブランチの`develop`ブランチに`merge`して新たなコミットが作成されました。

<a id="markdown-プルのコンフリクトに対処する" name="プルのコンフリクトに対処する"></a>
### プルのコンフリクトに対処する
リモートリポジトリのコミットで行われた変更と、ローカルリポジトリのコミットで行った変更が競合してしまった時、`pull`したときにリモート追跡ブランチとローカルブランチで`conflict`が起こります。下記の場合、`remotes/develop`と`develop`ブランチが競合しています。

<img width="450" alt="pull_conflict.png" src="">

`push`は、`fetch`と`merge`なので、`merge`の`conflict`と同じ対処方法で解決できます。
今回は`develop`が`remotes/develop`を`merge`するので、作業ブランチは`develop`です。
原因のフォルダを開いて、修正したら`commit`を行いましょう。


<a id="markdown-プルリクエストの正体" name="プルリクエストの正体"></a>
### (余談)プルリクエストの正体
基本的にリモートとローカルの関係は、リモートリポジトリからローカルリポジトリに`pull`し、ローカルリポジトリからリモートリポジトリに`push`することになります。
ですが、GitHubをはじめとするサービスには、ローカルリポジトリからリモートリポジトリに`pull`するという仕組みをとっています。
というのは、開発者の個人の判断`main`ブランチなどに`push`して、リモートリポジトリを更新してしまうと誰もチェックできずに大きな障害が発生する可能性があります。
一旦上位の開発者がコードをレビューするプロセスを挟むのが`pull request`です。

<img width="450" alt="pull_request.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/49ceb654-292c-0b89-f537-1a771be1a7cd.png">

**(info)**

`pull`:`fetch` + `merge`。`pull`は、リモートリポジトリの状態をローカルリポジトリに反映させること。


<a id="markdown-便利な機能" name="便利な機能"></a>
## 便利な機能

<a id="markdown-リバート" name="リバート"></a>
### リバート
前回行ったコミットを訂正するための`commit`することを`revert`と言います。
例えば`m9sgLe`で`second.txt`をローカルリポジトリに追加したとしましょう。

`revert`を行うと、その`commit`が取り消されて`second.txt`はローカルリポジトリから無くなります。

<img width="450" alt="revert.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/255417d6-a05f-b68c-89eb-436ec1b92c7c.png">

`revert`の良さは、`commit`を残せることです。後ほど紹介する`reset`と区別しましょう。


<a id="markdown-スタッシュ" name="スタッシュ"></a>
### スタッシュ
変更ファイルがあると他のブランチに移動できないので、`commit`までするか変更を破棄してしまうかを選択しなければなりません。
そんな時に活躍するのが`stash`です。
ワーキングディレクトリやステージングエリアにあるファイルを一時避難させることができます。

<img width="450" alt="stash.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/a8e83f88-bce5-560b-d496-80bea2918a65.png">

他のブランチに移動したい時、`stash`し、帰ってきたら`stash pop`で避難したファイルを取り戻して作業を再開します。


<a id="markdown-チェリーピック" name="チェリーピック"></a>
### チェリーピック
任意のコミットを現在のブランチに持ってきてコミットを作ることを`cherry-pick`と言います。
まさにいいとこ取りのような機能です。

<img width="450" alt="cherry.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/27601d74-62c8-2814-dfc6-3e31e6abf246.png">


以前に`feature`ブランチで実装した**〇〇〇な機能だけ**持ってきて、現在`develop`ブランチの作業に使用したいときなどに使用します。

<a id="markdown-HEADを使いこなす" name="HEADを使いこなす"></a>
### HEADを使いこなす
HEADは、現在作業中のブランチのポインタと説明しました。
また、ブランチはコミットを指すポインタだ、とも説明しました。

下の図を見てください。

<img width="450" alt="head.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/b4c0651f-a613-f858-9f53-7bf9de227f5d.png">

HEADが指すものは`develop`ブランチ、`develop`ブランチが指すもの`eaPk76`というコミットです。
つまり、この状況でのHEADは、`eaPk76`のコミットを指しているということになります。

よくGitのドキュメントや記事などに、コマンドの後ろに`HEAD`を使うのを見たことがありませんか？
例えば、`git revert HEAD`など。
これは、`HEAD`をコミットと置き換えることができるから実現できるコマンドなのです。

<a id="markdown-リセット" name="リセット"></a>
### (おまけ)リセット
現在の最新のコミットを取り消してもう一度作業することを`reset`といいます。

`--soft`オプションを使用すると`add`した直後に戻ることができます。
`--mixed`オプションを使用すると、ワーキングディレクトリで作業していた段階に戻ることができます。
`--hard <commit>`オプションを使用すると、戻るコミット地点までのすべてのコミットを削除し、指定コミットに`HEAD`を移動させます。

<img width="450" alt="reset.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/6d4a7eee-ce0a-4af5-ef0f-5541cb6b66b2.png">


`reset`は**完全にコミットが削除**されるので、特に'--hard'オプションについては、余程のことがなければ使用しないことを推奨します。

<a id="markdown-終わりに" name="終わりに"></a>
## 終わりに

<a id="markdown-Git以外のソースコードの管理" name="Git以外のソースコードの管理"></a>
### Git以外のソースコードの管理
Gitと同じ歴史を持ったMercurialというサービスがあります。
特徴は、Gitのような柔軟性を犠牲に非常にシンプルなコマンドラインインターフェース(CLI)を採用していることです。
最近だと、このMercurialをベースにMeta社がSaplingという新しいソースコード管理システムをオープンソースで公開されましたね。
また今度、ちょっと触ってみて感想を書いてみたいなと思います。

<a id="markdown-リモートリポジトリの居場所" name="リモートリポジトリの居場所"></a>
### リモートリポジトリの居場所
リモートリポジトリ用のサーバーを貸してくれるサービスをホスティングサービスと言います。
代表的なものであれば、GitHub, Bitbucket、プライベートに使用するAws Code Commitなどがあります。
GitとGit Hubは、全く別物です。
ちなみに、上で書いた通り、リモートリポジトリ用のサーバーは自分達のサーバーでも大丈夫です。

<a id="markdown-ポインタ" name="ポインタ"></a>
### ポインタ
C言語のようなメモリを直接扱うプログラミングに触れたことがある方は、なんとなく「ポインタ」の意味がわかると思います。
一方で、初学者の方にとって、すごく曖昧なものに感じると思います。

コミットオブジェクトは、リポジトリ内に保存されていると言いました。
リポジトリ内に、たくさんのコミットオブジェクトが溢れていたら、どのように欲しいオブジェクトを選ぶことができるでしょうか。

それは、特定のコミットオブジェクトありかを突き止めるラベル(住所)が必要になります。

<img width="600" alt="pointer.png" src="https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/2918231/1d4f5378-f935-7703-b5bb-93ac76b73b28.png">


「ポインタ」は、そのラベルを忘れないように指差してくれる貴重なデータというわけです。

ちなみにラベルは、ハッシュ関数と呼ばれる関数を通じて不思議な文字列へと変換されたものを使います。
気になる方は、[Gitのハッシュ値の求め方](https://engineering.mercari.com/blog/entry/2016-02-08-173000/)を参考にしてください。

<a id="markdown-さらにGitを理解するために" name="さらにGitを理解するために"></a>
### さらにGitを理解するために
この記事で言及できなかったことがたくさんあります。
Gitのコアの部分はシンプルなキー・バリュー型データストアであることや、バリューとなるGitオブジェクトの詳細、そしてオブジェクトをどのように扱うかなど。
いつか完全攻略したいと思います。


<a id="markdown-参考サイト" name="参考サイト"></a>
# 参考サイト
- [Git Documentation](https://git-scm.com/doc)
- [Learn git concepts, not commands](https://dev.to/unseenwizzard/learn-git-concepts-not-commands-4gjc)
- [図解 Git](https://marklodato.github.io/visual-git-guide/index-ja.html)
- [いまさらだけどGitを基本から分かりやすくまとめてみた](https://qiita.com/gold-kou/items/7f6a3b46e2781b0dd4a0)
- [git add ってなんのためにやるの？](https://kray.jp/blog/expound-git-add/)
