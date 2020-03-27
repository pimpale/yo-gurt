use std::io::stdin;
use conllu::io::Reader;
use conllu::io::ReadSentence;
use std::io::BufRead;
use bumpalo::Bump;
use std::cell::RefCell;
use yogurt::tokenizer;

fn main() {
    let mut arena = Bump::new();
    let english_ruleset = tokenizer::RuleSet::english(&mut arena);

    for line in stdin().lock().lines() {
        if let Ok(string) = line {
            for tok in english_ruleset.tokenize(&string) {
                dbg!(tok);
            }
        }
    }
}

fn doread() {
    let stdin = stdin();
    let reader = Reader::new(stdin.lock());
    for sentence_result in reader.sentences() {
        if let Ok(sentence) = sentence_result {
            println!("OK!!!");
            println!("{}", sentence);
        } else if let Err(err) = sentence_result {
            println!("ERR!!!");
            println!(" {} ", err);
        } else {
            println!("NANI!!!");
        }
    }
}
