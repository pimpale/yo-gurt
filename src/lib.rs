#![feature(hash_set_entry)]

pub mod tokenizer;
pub mod parser;


// 'doc is the lifetime of the doc
// 'vocab is the lifetime of the vocab
#[derive(Debug)]
pub struct Token<'doc, 'vocab> {
    pub string: &'doc str,
    pub norm: Option<&'vocab str>,
}

