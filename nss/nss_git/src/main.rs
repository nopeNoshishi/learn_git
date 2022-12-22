mod blob;
use blob::Blob;

use std::fs::File;
use std::io::prelude::*;
use clap::{App, Arg};


fn main() {
    // 引数の解析
    let subcommand1 = App::new("blob")
                        .arg(Arg::with_name("filename")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1));
    
    let command = App::new("nss_git")
                        .version("0.0.1")
                        .author("Noshishi. <noshishi@gmail.com>")
                        .about("Original Git")
                        .subcommand(subcommand1);
    
    match command.get_matches().subcommand() {
        Some(("blob", sub_m)) => {
            let filename = sub_m.value_of("INPUT").unwrap();
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
        _ => todo!()
    }
}







