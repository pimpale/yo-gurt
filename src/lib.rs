
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
    RP,      // paricle
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

use std::collections::HashSet;

type VocabIndex = usize;

pub enum SpecialRule {
    OneToMany {
        origs:String,
        results:Vec<String>
    },
    ManyToOne {
        origs:Vec<String>,
        result:String
    },
}

pub struct RuleSet {
    // General Prefixes
    prefix: HashSet<String>, // Prefixes
    // General Suffixes
    suffix: HashSet<String>, // Suffixes (n't, 've, etc)
    exact: HashSet<String>, // N.Y.. U.S., etc
}

impl RuleSet {
    // Optionally returns a token
    pub fn get_exact<'a>(&self, string: &'a str) -> Option<&'a str> {
        if self.exact.contains(string) {
            Some(string)
        } else {
            None
        }
    }
    // Matches the longest prefix
    pub fn get_prefix_remainder<'a>(&self, string: &'a str) -> Option<(&'a str, &'a str)> {
        for i in string.len()..0 {
            if self.prefix.contains(&string[..i]) {
                return Some( ( &string[..i], &string[i..]) );
            }
        }
        return None;
    }

    // Matches the longest suffix
    pub fn get_suffix_remainder<'a>(&self, string: &'a str) -> Option<(&'a str, &'a str)> {
        for i in string.len()..0 {
            if self.suffix.contains(&string[..i]) {
                return Some( ( &string[i..], &string[..i] ) );
            }
        }
        return None;
    }
}

pub struct Vocab {}

type NodeIndex = usize;

pub enum Node<'a> {
    Root,
    Token {
        string: &'a str,
        lemma: VocabIndex,
        pos: PartOfSpeech,
        parent: NodeIndex,
    },
}

// Returns an optional span if the rule is matched
// pub fn matchRule(string:&str, rule:&Rule) -> bool {
//     match(rule.kind) {
//         FullToken => rule.text == string,
//         Prefix => rule.text == string[0..min(rule.text.len(), string.len()),
//         Suffix => rule
//
//     }
// }

// Tokenize LOWERCASE string
// Uses spacy algorithm
pub fn tokenize<'a>(string: &'a String, ruleset: &RuleSet) -> Vec<&'a str> {
    let mut tokens = Vec::new();

    for s in string.split_whitespace() {
        let mut substr = s;
        loop {
            if let Some(token) = ruleset.get_exact(&substr) {
                tokens.push(token);
                // this will cause us to start viewing the next substr
                break;
            } else if let Some((token, remainder)) = ruleset.get_prefix_remainder(&substr) {
                tokens.push(token);
                substr = remainder;
                continue;
            } else if let Some((token, remainder)) = ruleset.get_suffix_remainder(&substr) {
                tokens.push(token);
                substr = remainder;
                continue;
            }
        }
    }

    tokens
}
