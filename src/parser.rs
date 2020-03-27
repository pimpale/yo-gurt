use super::Token;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

pub enum Moves {
    Shift,
    LeftArc,
    RightArc,
}


#[derive(std::hash::Hash)]
#[derive(std::cmp::Eq)]
#[derive(std::cmp::PartialEq)]
struct TokenArc {
    parent: Option<TokenIndex>,
    child:TokenIndex,
}

#[derive(std::hash::Hash)]
#[derive(std::cmp::Eq)]
#[derive(std::cmp::PartialEq)]
pub struct TokenIndex {
    value:usize,
}

pub struct Parser<'doc, 'vocab> {
    token_backing:Vec<Token<'doc, 'vocab>>,
    queue:VecDeque<TokenIndex>,
    stack:Vec<TokenIndex>,
    arcs:HashSet<TokenArc>,
}

impl<'doc, 'vocab> Parser<'doc, 'vocab> {

    // Create new parse tree
    pub fn new(sentence:Vec<Token<'doc, 'vocab>>) -> Parser<'doc, 'vocab> {
        Parser {
            queue: (0..sentence.len()).map(|value| TokenIndex {value}).collect(),
            token_backing: sentence,
            stack: Vec::new(),
            arcs: HashSet::new()
        }
    }

    fn queue_empty(&self) -> bool {
        self.queue.len() == 0
    }

    // Make sure queue is not empty
    fn shift(&mut self) -> () {
        self.stack.push(self.queue.pop_front().unwrap());
    }

    // Ensure there are 2 nodes on the stack
    pub fn left_arc(&mut self, label:&str) {
        
    }

}
