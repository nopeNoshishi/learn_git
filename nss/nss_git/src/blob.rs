// blob objectの作成する
use sha1::{Digest, Sha1};
use std::fmt;


pub struct Blob {
    pub size: usize,
    pub content: String
}

impl Blob {
    // contentからの作成
    pub fn new(content: String) -> Self {
        Self {
            size: content.len(),
            content
        }
    }

    // bytes文字列からの作成
    pub fn from(bytes: &[u8]) -> Option<Self> {
        let content = String::from_utf8(bytes.to_vec());

        match content {
            Ok(content) => Some(Self {
            size: content.len(),
            content,            
            }),
            _ => None
        }
    }

    // 書き込み用にbytesに変換
    pub fn as_bytes(&self) -> Vec<u8> {
        let header = format!("blob {}\0", self.size);
        println!("blob header: {:?}", &header);
        let store = format!("{}{}", header, self.to_string());
        println!("blob store context: {:?}", &store);

        Vec::from(store.as_bytes())
    }

    // hash値の計算
    pub fn calc_hash(&self) -> Vec<u8> {
        Vec::from(Sha1::digest(&self.as_bytes()).as_slice())
    }
}

impl fmt::Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
