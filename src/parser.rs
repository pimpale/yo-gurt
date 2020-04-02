/*
use super::tokenizer::Token;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::hash::Hash;

#[derive(Hash, Eq, PartialEq)]
pub enum GrammaticalFunction {
    // Clausal argument relations
    NSUBJ, // Nominal Subject
    DOBJ,  // Direct object
    IOBJ,  // Indirect Object
    CCOMP, // clausal complement
    XCOMP, // open clausal complement
    // Nominal Modifier Relations
    NMOD,   // Nominal modifier
    AMOD,   // Adjectival modifier
    NUMMOD, // Numeric Modifier
    APPOS,  // Appositional modifier
    DET,    // Determiner
    // Other notable relations
    CONJ, // conjunct
    CC,   // coordinating conjunction
}

pub enum Moves {
    Shift,
    LeftArc,
    RightArc,
}

#[derive(Hash, Eq, PartialEq)]
struct TokenArc {
    parent: TokenIndex,
    child: TokenIndex,
    label: GrammaticalFunction
}

#[derive(Hash, Clone, Copy, Eq, PartialEq)]
pub enum TokenIndex {
    Root,
    Value(usize),
}

pub struct Parser<'doc, 'vocab> {
    token_backing: Vec<Token<'doc, 'vocab>>,
    queue: VecDeque<TokenIndex>,
    stack: Vec<TokenIndex>,
    arcs: HashSet<TokenArc>,
}

impl<'doc, 'vocab> Parser<'doc, 'vocab> {
    // Create new parse tree
    pub fn new(sentence: Vec<Token<'doc, 'vocab>>) -> Parser<'doc, 'vocab> {
        Parser {
            queue: (0..sentence.len())
                .map(|value| TokenIndex::Value(value))
                .collect(),
            token_backing: sentence,
            stack: vec![TokenIndex::Root],
            arcs: HashSet::new(),
        }
    }

    fn valid_moves(&self) -> Vec<Moves> {
        let mut valid = Vec::new();
        if self.queue.len() > 0 {
            valid.push(Moves::Shift);
        }

        if self.stack.len() > 1 {
            valid.push(Moves::RightArc);

            if self.stack[self.stack.len() - 2] != TokenIndex::Root {
                valid.push(Moves::LeftArc);
            }
        }
        return valid;
    }

    // pseudocode taken from: https://www.diva-portal.org/smash/get/diva2:661423/FULLTEXT01.pdf

    // Dequeues the topmost node onto the stack
    // Preconditions:
    // Make sure queue is not empty before using
    pub fn shift(&mut self) -> () {
        self.stack.push(self.queue.pop_front().unwrap());
    }

    // adds new arc with label from the topmost node on the stack to second topmost node on the
    // stack, then removes the second topmost node
    // Preconditions:
    // Ensure there are 2 nodes on the stack
    // Ensure that Root is not the second element on the stack
    pub fn left_arc(&mut self, label: GrammaticalFunction) {
        let parent = *self.stack.last().unwrap();
        let child = self.stack.remove(self.stack.len() - 2);
        if child == TokenIndex::Root {
            panic!("Can't make root a child");
        }
        self.arcs.insert(TokenArc {
            parent,
            child,
            label,
        });
    }

    // adds new arc with label from the second topmost node on the stack to the topmost node and
    // removes the topmost node
    // Preconditions:
    // Ensure there are 2 nodes on the stack
    pub fn right_arc(&mut self, label: GrammaticalFunction) {
        let parent = self.stack[self.stack.len() - 2];
        let child = self.stack.pop().unwrap();
        // it's not possible for root to be a child here because we guarantee that there are 2
        // nodes on the stack, and root node is always the bottommost node
        self.arcs.insert(TokenArc {
            parent,
            child,
            label,
        });
    }
}
*/
