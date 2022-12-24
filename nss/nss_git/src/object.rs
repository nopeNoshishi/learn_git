// Git Objectを作成する
use sha1::{Digest, Sha1};
use std::fmt;

// blob objectにファイル名を追加する
pub struct File {
    pub mode: usize,
    pub path: String,
    pub blob_hash: Vec<u8>
}

impl File {
    pub fn from(header: &[u8], hash: &[u8]) -> Option<self> {
        let header = String::from_utf8(header.to_vec())

    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let header = format!("path {}\0", self.size);
        println!("blob header: {:?}", &header);
        let store = format!("{}{}", header, self.to_string());
        println!("blob store context: {:?}", &store);

        Vec::from(store.as_bytes())
    }
}


pub struct Tree {
    pub contents: Vec<File>,
}


impl Tree {
    pub fn new(contents: Vec<File>) -> Self {
        Self {
            contents
        }
    }

    // 書き込み用にbytesに変換
    pub fn as_bytes(&self) -> Vec<u8> {
        let header = format!("tree {}\0", self.size);
        println!("header: {:?}", &header);
        let store = format!("{}{}", header, self.to_string());
        println!("store: {:?}", &store);

        Vec::from(store.as_bytes())
    }

    // hash値の計算
    pub fn calc_hash(&self) -> Vec<u8> {
        Vec::from(Sha1::digest(&self.as_bytes()).as_slice())
    }

}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.contents)
    }
}