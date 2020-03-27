#![feature(hash_set_entry)]

pub mod tokenizer;
pub mod parser;

// Definitions from here:
// https://web.stanford.edu/~jurafsky/slp3/8.pdf
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
pub struct Token<'doc, 'vocab> {
    pub string: &'doc str,
    pub norm: Option<&'vocab str>,
}

