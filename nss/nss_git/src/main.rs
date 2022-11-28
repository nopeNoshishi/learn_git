mod object;
use object::Blob;

use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    // 引数の解析
    let args: Vec<String> = env::args().collect();

    // 第一引数はファイル名
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");

    let mut content = String::new();
    f.read_to_string(&mut content)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");

    // blob objectをcontentから作成
    let blob = Blob::new(content);
    println!("file content \n{:}", &blob);

    // hashのバイナリを16進ダンプ
    let hash = blob.calc_hash();
    println!("{}", hex::encode(hash));

}







