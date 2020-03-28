use super::Token;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter;

// Definitions from here:
// https://web.stanford.edu/~jurafsky/slp3/8.pdf
#[derive(std::hash::Hash, Clone, Copy, std::cmp::Eq, std::cmp::PartialEq)]
pub enum PartOfSpeech {
    CC,      // coordinating conjunction
    CD,      // cardinal number
    DT,      // determiner
    EX,      // existential there
    FW,      // foreign word
    IN,      // preposition/subordinating conjunction
    JJ,      // adjective
    JJR,     // comparative adjective
    JJS,     // superlative adjective
    LS,      // list item marker
    MD,      // modal
    NN,      // singular or mass noun
    NNS,     // noun plural
    NNP,     // proper noun, singular
    NNPS,    // proper noun, plural
    PDT,     // predeterminer
    POS,     // possessive ending
    PRP,     // personal pronoun
    PRP_S,   // possessive pronoun
    RB,      // adverb
    RBR,     // comparative adverb
    RBS,     // superlative adverb
    RP,      // particle
    SYM,     // symbole
    TO,      // to
    UH,      // interjection
    VB,      // verb base form
    VBD,     // verb past tense
    VBG,     // verb gerund
    VBN,     // verb past participle
    VBP,     // verb non 3sg persent
    VBZ,     // verb 3sg present
    WDT,     // wh determine
    WP,      // wh pronoun
    WP_S,    // wh posess
    WRB,     // wh adverb
    DOLLAR,  // $
    HASH,    // #
    LQUOTE,  // "
    RQUOTE,  // "
    LPAREN,  // (
    RPAREN,  // )
    COMMA,   // ,
    ENDPUNC, // .
    MIDPUNC, // ;
}

pub enum Moves {
    Shift,
    LeftArc,
    RightArc,
}

#[derive(std::hash::Hash, std::cmp::Eq, std::cmp::PartialEq)]
struct TokenArc {
    parent: TokenIndex,
    child: TokenIndex,
    label: PartOfSpeech
}

#[derive(std::hash::Hash, Clone, Copy, std::cmp::Eq, std::cmp::PartialEq)]
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
    pub fn left_arc(&mut self, label: PartOfSpeech) {
        let parent = *self.stack.last().unwrap();
        let child = self.stack.remove(self.stack.len() - 2);
        if child == TokenIndex::Root {
            panic!("Can't make root a child");
        }
        self.arcs.insert(TokenArc { parent, child, label});
    }

    // adds new arc with label from the second topmost node on the stack to the topmost node and
    // removes the topmost node
    // Preconditions:
    // Ensure there are 2 nodes on the stack
    pub fn right_arc(&mut self, label: PartOfSpeech) {
        let parent = self.stack[self.stack.len() - 2];
        let child = self.stack.pop().unwrap();
        // it's not possible for root to be a child here because we guarantee that there are 2
        // nodes on the stack, and root node is always the bottommost node
        self.arcs.insert(TokenArc { parent, child, label});
    }
}
