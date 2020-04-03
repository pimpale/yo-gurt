use super::lexemizer::Lexeme;
use std::collections::HashMap;

// Definitions from here:
// https://web.stanford.edu/~jurafsky/slp3/8.pdf
#[derive(Debug, std::hash::Hash, Clone, Copy, std::cmp::Eq, std::cmp::PartialEq)]
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

// 'doc is the lifetime of the doc
// 'vocab is the lifetime of the vocab
#[derive(Debug)]
pub struct Token {
    pub lemma:String,
    pub part_of_speech: PartOfSpeech,
}

// Lives as long as the model is loaded
// Don't want to allocate a whole new vec for each feature...
enum Feature<'model>{
    Bias,
    Suffix {
        word_index:i8,  // refers to the index of the word. Current word is 0. Next word is 1, Previous word is -1
        value:&'model [u8], // the chars making up the first part
    },
    Prefix {
        word_index:i8,
        value:&'model [u8], // the chars making up the first part
    },
    Tag {
        word_index:i8,
        value:PartOfSpeech,
    },
    TagTag {
        word_indexes:(i8, i8),
        values: (PartOfSpeech, PartOfSpeech)
    }
}

struct Perceptron<'model> {
    weights:HashMap<Feature<'model>, HashMap<PartOfSpeech, f64>>,
}

impl<'model> Perceptron {
    
}

pub fn tokenize(values:Vec<Lexeme>) -> Vec<Token> {
    
}

