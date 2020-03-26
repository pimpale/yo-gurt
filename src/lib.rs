#![feature(hash_set_entry)]

use std::collections::HashMap;
use std::collections::HashSet;
use typed_arena::Arena;

mod english;

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

pub struct Token<'doc, 'vocab> {
    string: &'doc str,
    lemma: &'vocab str,
}

pub struct RuleSet<'vocab> {
    // General Prefixes
    general_prefix: HashSet<&'vocab str>, // Prefixes
    // General Suffixes
    general_suffix: HashSet<&'vocab str>, // Suffixes (n't, 've, etc)
    special_expand: HashMap<&'vocab str, Vec<&'vocab str>>, // N.Y.. U.S., etc
}

impl<'vocab> RuleSet<'vocab> {
    pub fn new(
        general_prefix: HashSet<&'vocab str>,
        general_suffix: HashSet<&'vocab str>,
        special_expand: HashMap<&'vocab str, Vec<&'vocab str>>,
    ) -> RuleSet<'vocab> {
        RuleSet {
            general_prefix,
            general_suffix,
            special_expand,
        }
    }

    // If there is an exact match between this string and a special expand,
    // We create a set of tokens with lemmas and the text
    pub fn special_expand<'doc>(&self, string: &'doc str) -> Option<Vec<Token<'doc, 'vocab>>> {
        let ret = self.special_expand.get(&string);
        if let Some(vs) = ret {
            Some(vs.iter().map(|lemma| Token { string, lemma }).collect())
        } else {
            None
        }
    }

    // Matches the longest prefix
    // Returns A remainder, and a prefix token
    pub fn general_prefix_remainder<'doc>(
        &self,
        string: &'doc str,
    ) -> Option<(Token<'doc, 'vocab>, &'doc str)> {
        for i in string.len()..0 {
            if let Some(prefix_lemma) = self.general_prefix.get(&string[..i]) {
                return Some((
                    Token {
                        string: &string[..i],
                        lemma: prefix_lemma,
                    },
                    &string[i..],
                ));
            }
        }
        return None;
    }

    // Matches the longest suffix
    pub fn general_suffix_remainder<'doc>(
        &self,
        string: &'doc str,
    ) -> Option<(Token<'doc, 'vocab>, &'doc str)> {
        for i in string.len()..0 {
            if let Some(suffix_lemma) = self.general_suffix.get(&string[i..]) {
                return Some((
                    Token {
                        string: &string[i..],
                        lemma: suffix_lemma,
                    },
                    &string[..i],
                ));
            }
        }
        return None;
    }
}

// Tokenize LOWERCASE string
// Uses spacy algorithm
pub fn tokenize<'doc, 'vocab>(
    string: &'doc String,
    ruleset: &'vocab RuleSet,
) -> Vec<Token<'doc, 'vocab>> {
    let mut tokens = Vec::new();

    for s in string.split_whitespace() {
        let mut substr = s;
        loop {
            if let Some(mut tokvec) = ruleset.special_expand(&substr) {
                tokens.append(&mut tokvec);
                // this will cause us to start viewing the next substr
                break;
            } else if let Some((token, remainder)) = ruleset.general_prefix_remainder(&substr) {
                tokens.push(token);
                substr = remainder;
                continue;
            } else if let Some((token, remainder)) = ruleset.general_suffix_remainder(&substr) {
                tokens.push(token);
                substr = remainder;
                continue;
            }
        }
    }
    tokens
}
