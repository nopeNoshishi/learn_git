mod blob;
use blob::Blob;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use clap::{Command, Arg};


fn main() {
    // 引数の解析
    let about = fs::read_to_string("description/about.txt").expect("file not found");
    let help = fs::read_to_string("description/help.txt").expect("file not found");
    
    let cmd = Command::new("nss")
                        .about(about)
                        .version("0.0.1")
                        .author("Noshishi. <noshishi@noshishi.com>")
                        .override_help(help)
                        .subcommand(Command::new("blob")
                            .about("Create blob")
                            .arg(Arg::new("filename")
                                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                                .required(true)
                                .help("Read file to covert blob object")
                                .value_name("FILE")));
    
    match cmd.get_matches().subcommand() {
        Some(("blob", sub_m)) => {
            let filename: &String = sub_m.get_one("filename").expect("`port`is required");
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
