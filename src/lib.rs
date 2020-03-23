use std::collections::HashMap;
use crc::crc64;

type StringHash = u64;

pub struct Vocabulary {
    string_store: HashMap<StringHash, String, crc64::Digest>
}

impl Vocabulary {
    pub fn new() -> Vocabulary {
        Vocabulary {
            // A Vocabulary is probably going to have at least 1k words
            string_store: HashMap::with_capacity_and_hasher(1000, crc64::Digest::new_with_initial(crc64::ECMA, 0u64))
        }
    }

    pub fn hash(&self) {

    }

}

pub struct Token {

}

pub fn greet() {
}
