use super::Token;
use bumpalo::Bump;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

// simple put utility method for arena
macro_rules! arena_put {
    ($arena:expr, $string:expr) => {
        (&*$arena.alloc_str($string))
    };
}

macro_rules! add_rule {
    ($arena:expr, $hashmap:expr, $key:expr, [$($val:expr),* $(,)?]) => {
        $hashmap.insert(arena_put!($arena, $key), vec![$( arena_put!($arena, $val)),*])
    };
}

macro_rules! add_exact_rule {
    ($arena:expr, $hashmap:expr, $key:expr) => {
        add_rule!($arena, $hashmap, $key, [$key])
    };
}

macro_rules! cat {
    ($str1:expr, $str2:expr) => {
        &([$str1, $str2].concat())
    };
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

    // Adapted from Spacy
    pub fn english(a: &'vocab mut Bump) -> RuleSet<'vocab> {
        // Prefixes
        let general_prefixes = HashSet::from_iter(vec![
            arena_put!(a, "("),
            arena_put!(a, ")"),
            arena_put!(a, "$"),
            arena_put!(a, "#"),
            arena_put!(a, "."),
            arena_put!(a, "'"),
            arena_put!(a, "\""),
            arena_put!(a, ".."),
            arena_put!(a, "..."),
            arena_put!(a, "&"),
            arena_put!(a, "@"),
            arena_put!(a, "?"),
        ]);

        let general_suffixes = HashSet::from_iter(vec![
            arena_put!(a, "("),
            arena_put!(a, ")"),
            arena_put!(a, "$"),
            arena_put!(a, "#"),
            arena_put!(a, "."),
            arena_put!(a, "'"),
            arena_put!(a, "\""),
            arena_put!(a, ".."),
            arena_put!(a, "..."),
            arena_put!(a, "&"),
            arena_put!(a, "@"),
            arena_put!(a, "?"),
        ]);

        let mut special = HashMap::new();
        add_rule!(a, special, "i'm", ["i", "am"]);
        add_rule!(a, special, "im", ["i", "am"]);
        add_rule!(a, special, "i'mma", ["i", "am", "going", "to"]);
        add_rule!(a, special, "imma", ["i", "am", "going", "to"]);

        for pronoun in ["i", "you", "he", "she", "it", "we", "they"].iter() {
            add_rule!(a, special, cat!(pronoun, "'ll"), [pronoun, "will"]);
            add_rule!(
                a,
                special,
                cat!(pronoun, "'ll've"),
                [pronoun, "will", "have"]
            );
            add_rule!(a, special, cat!(pronoun, "llve"), [pronoun, "will", "have"]);
            add_rule!(a, special, cat!(pronoun, "'d"), [pronoun, "would"]);
            add_rule!(a, special, cat!(pronoun, "d"), [pronoun, "would"]);
            add_rule!(
                a,
                special,
                cat!(pronoun, "'d've"),
                [pronoun, "would", "have"]
            );
            add_rule!(a, special, cat!(pronoun, "dve"), [pronoun, "would", "have"]);
        }

        for pronoun in ["i", "you", "we", "they"].iter() {
            add_rule!(a, special, cat!(pronoun, "'ve"), [pronoun, "have"]);
            add_rule!(a, special, cat!(pronoun, "ve"), [pronoun, "have"]);
        }

        for pronoun in ["you", "we", "they"].iter() {
            add_rule!(a, special, cat!(pronoun, "'re"), [pronoun, "are"]);
            // were, not we're
            if pronoun != &"we" {
                add_rule!(a, special, cat!("re", pronoun), [pronoun, "are"]);
            }
        }

        // Posessives
        for pronoun in ["it", "he", "she"].iter() {
            add_rule!(a, special, &cat!(pronoun, "'s"), [pronoun, "'s"]);
            add_rule!(a, special, &cat!(pronoun, "s"), [pronoun, "'s"]);
            // "it" is special case
            if pronoun == &"it" {
                add_rule!(a, special, "its", ["it", "'s"]);
                add_rule!(a, special, "it's", ["it", "is"]);
            }
        }

        // W words, relative pronouns, and prepositions
        for word in [
            "who", "what", "when", "where", "why", "how", "there", "that",
        ]
        .iter()
        {
            // Possessives
            add_rule!(a, special, cat!(word, "'s"), [word, "'s"]);
            add_rule!(a, special, cat!(word, "s"), [word, "'s"]);
            // will
            add_rule!(a, special, cat!(word, "'ll"), [word, "'ll"]);
            add_rule!(a, special, cat!(word, "ll"), [word, "'ll"]);
            // have
            add_rule!(a, special, cat!(word, "'ve"), [word, "have"]);
            add_rule!(a, special, cat!(word, "ve"), [word, "have"]);
            // will have
            add_rule!(a, special, cat!(word, "'ll've"), [word, "will", "have"]);
            add_rule!(a, special, cat!(word, "llve"), [word, "will", "have"]);
            // would
            add_rule!(a, special, cat!(word, "'d"), [word, "would"]);
            add_rule!(a, special, cat!(word, "d"), [word, "would"]);
            // would have
            add_rule!(a, special, cat!(word, "'d've"), [word, "would", "have"]);
            add_rule!(a, special, cat!(word, "dve"), [word, "would", "have"]);
            // are
            add_rule!(a, special, cat!(word, "'re"), [word, "are"]);
            add_rule!(a, special, cat!(word, "re"), [word, "are"]);
        }

        for word in [
            "ca", "can", "could", "do", "does", "did", "had", "may", "might", "must", "need",
            "ought", "sha", "should", "wo", "would",
        ]
        .iter()
        {
            add_rule!(a, special, cat!(word, "n't"), [word, "not"]);
            add_rule!(a, special, cat!(word, "nt"), [word, "not"]);
            add_rule!(a, special, cat!(word, "n't've"), [word, "not", "have"]);
            add_rule!(a, special, cat!(word, "ntve"), [word, "not", "have"]);
        }

        for word in ["could", "might", "must", "should", "would"].iter() {
            add_rule!(a, special, cat!(word, "'ve"), [word, "have"]);
            add_rule!(a, special, cat!(word, "ve"), [word, "have"]);
        }

        for word in ["ai", "are", "is", "was", "were", "have", "has", "dare"].iter() {
            add_rule!(a, special, cat!(word, "n't"), [word, "not"]);
            add_rule!(a, special, cat!(word, "nt"), [word, "not"]);
        }

        // other contractions/abbreviations
        add_rule!(a, special, "y'all", ["you", "all"]);
        add_rule!(a, special, "yall", ["you", "all"]);
        add_rule!(a, special, "cannot", ["can", "not"]);
        add_rule!(a, special, "gonna", ["going", "to"]);
        add_rule!(a, special, "gotta", ["got", "to"]);
        add_rule!(a, special, "let's", ["let", "us"]);
        add_rule!(a, special, "lets", ["let", "us"]);
        add_rule!(a, special, "'s", ["'s"]);
        add_rule!(a, special, "and/or", ["and/or",]);
        add_rule!(a, special, "w/o", ["without"]);
        add_rule!(a, special, "'re", ["are"]);
        add_rule!(a, special, "'cause", ["because"]);
        add_rule!(a, special, "'cos", ["because"]);
        add_rule!(a, special, "'coz", ["because"]);
        add_rule!(a, special, "'cuz", ["because"]);
        add_rule!(a, special, "'bout", ["about"]);
        add_rule!(a, special, "ma'am", ["madam"]);
        add_rule!(a, special, "o'clock", ["o'clock"]);
        add_rule!(a, special, "lovin'", ["loving"]);
        add_rule!(a, special, "lovin", ["loving"]);
        add_rule!(a, special, "havin'", ["having"]);
        add_rule!(a, special, "havin", ["having"]);
        add_rule!(a, special, "doin'", ["doing"]);
        add_rule!(a, special, "doin", ["doing"]);
        add_rule!(a, special, "goin'", ["going"]);
        add_rule!(a, special, "goin", ["going"]);
        add_rule!(a, special, "mt.", ["mount"]);
        add_rule!(a, special, "ak.", ["alaska"]);
        add_rule!(a, special, "ala.", ["alabama"]);
        add_rule!(a, special, "apr.", ["april"]);
        add_rule!(a, special, "ariz.", ["arizona"]);
        add_rule!(a, special, "ark.", ["arkansas"]);
        add_rule!(a, special, "aug.", ["august"]);
        add_rule!(a, special, "calif.", ["california"]);
        add_rule!(a, special, "colo.", ["colorado"]);
        add_rule!(a, special, "conn.", ["connecticut"]);
        add_rule!(a, special, "dec.", ["december"]);
        add_rule!(a, special, "del.", ["delaware"]);
        add_rule!(a, special, "feb.", ["february"]);
        add_rule!(a, special, "fla.", ["florida"]);
        add_rule!(a, special, "ga.", ["georgia"]);
        add_rule!(a, special, "ia.", ["iowa"]);
        add_rule!(a, special, "id.", ["idaho"]);
        add_rule!(a, special, "ill.", ["illinois"]);
        add_rule!(a, special, "ind.", ["indiana"]);
        add_rule!(a, special, "jan.", ["january"]);
        add_rule!(a, special, "jul.", ["july"]);
        add_rule!(a, special, "jun.", ["june"]);
        add_rule!(a, special, "kan.", ["kansas"]);
        add_rule!(a, special, "kans.", ["kansas"]);
        add_rule!(a, special, "ky.", ["kentucky"]);
        add_rule!(a, special, "la.", ["louisiana"]);
        add_rule!(a, special, "mar.", ["march"]);
        add_rule!(a, special, "mass.", ["massachusetts"]);
        add_rule!(a, special, "may.", ["may"]);
        add_rule!(a, special, "mich.", ["michigan"]);
        add_rule!(a, special, "minn.", ["minnesota"]);
        add_rule!(a, special, "miss.", ["mississippi"]);
        add_rule!(a, special, "n.c.", ["north carolina"]);
        add_rule!(a, special, "n.d.", ["north dakota"]);
        add_rule!(a, special, "n.h.", ["new hampshire"]);
        add_rule!(a, special, "n.j.", ["new jersey"]);
        add_rule!(a, special, "n.m.", ["new mexico"]);
        add_rule!(a, special, "n.y.", ["new york"]);
        add_rule!(a, special, "neb.", ["nebraska"]);
        add_rule!(a, special, "nebr.", ["nebraska"]);
        add_rule!(a, special, "nev.", ["nevada"]);
        add_rule!(a, special, "nov.", ["november"]);
        add_rule!(a, special, "oct.", ["october"]);
        add_rule!(a, special, "okla.", ["oklahoma"]);
        add_rule!(a, special, "ore.", ["oregon"]);
        add_rule!(a, special, "pa.", ["pennsylvania"]);
        add_rule!(a, special, "s.c.", ["south carolina"]);
        add_rule!(a, special, "sep.", ["september"]);
        add_rule!(a, special, "sept.", ["september"]);
        add_rule!(a, special, "tenn.", ["tennessee"]);
        add_rule!(a, special, "va.", ["virginia"]);
        add_rule!(a, special, "wash.", ["washington"]);
        add_rule!(a, special, "wis.", ["wisconsin"]);

        // yet more abbreviations
        add_exact_rule!(a, special, "'d");
        add_exact_rule!(a, special, "a.m.");
        add_exact_rule!(a, special, "adm.");
        add_exact_rule!(a, special, "bros.");
        add_exact_rule!(a, special, "co.");
        add_exact_rule!(a, special, "corp.");
        add_exact_rule!(a, special, "d.c.");
        add_exact_rule!(a, special, "dr.");
        add_exact_rule!(a, special, "e.g.");
        add_exact_rule!(a, special, "gen.");
        add_exact_rule!(a, special, "gov.");
        add_exact_rule!(a, special, "i.e.");
        add_exact_rule!(a, special, "inc.");
        add_exact_rule!(a, special, "jr.");
        add_exact_rule!(a, special, "ltd.");
        add_exact_rule!(a, special, "md.");
        add_exact_rule!(a, special, "messrs.");
        add_exact_rule!(a, special, "mo.");
        add_exact_rule!(a, special, "mont.");
        add_exact_rule!(a, special, "mr.");
        add_exact_rule!(a, special, "mrs.");
        add_exact_rule!(a, special, "ms.");
        add_exact_rule!(a, special, "p.m.");
        add_exact_rule!(a, special, "ph.d.");
        add_exact_rule!(a, special, "prof.");
        add_exact_rule!(a, special, "rep.");
        add_exact_rule!(a, special, "rev.");
        add_exact_rule!(a, special, "sen.");
        add_exact_rule!(a, special, "st.");
        add_exact_rule!(a, special, "vs.");
        add_exact_rule!(a, special, "v.s.");

        RuleSet::new(general_prefixes, general_suffixes, special)
    }

    // If there is an exact match between this string and a special expand,
    // We create a set of tokens with norms and the text
    pub fn special_expand<'doc>(&self, string: &'doc str) -> Option<Vec<Token<'doc, 'vocab>>> {
        let ret = self.special_expand.get(&string);
        if let Some(vs) = ret {
            Some(
                vs.iter()
                    .map(|norm| Token {
                        string,
                        norm: Some(norm),
                    })
                    .collect(),
            )
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
        for i in (1..string.len()).rev() {
            if let Some(prefix_norm) = self.general_prefix.get(&string[..i]) {
                return Some((
                    Token {
                        string: &string[..i],
                        norm: Some(prefix_norm),
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
        for i in 1..string.len() {
            if let Some(suffix_norm) = self.general_suffix.get(&string[i..]) {
                return Some((
                    Token {
                        string: &string[i..],
                        norm: Some(suffix_norm),
                    },
                    &string[..i],
                ));
            }
        }
        return None;
    }

    // Tokenize LOWERCASE string
    // Uses spacy algorithm
    pub fn tokenize<'doc>(&self, string: &'doc String) -> Vec<Token<'doc, 'vocab>> {
        let mut tokens = Vec::new();

        for s in string.split_whitespace() {
            let mut substr = s;
            loop {
                if let Some(mut tokvec) = self.special_expand(&substr) {
                    tokens.append(&mut tokvec);
                    // this will cause us to start viewing the next substr
                    break;
                } else if let Some((token, remainder)) = self.general_prefix_remainder(&substr) {
                    tokens.push(token);
                    substr = remainder;
                    continue;
                } else if let Some((token, remainder)) = self.general_suffix_remainder(&substr) {
                    tokens.push(token);
                    substr = remainder;
                    continue;
                }
                // If we can't do anything with it, push token
                tokens.push(Token {
                    string: substr,
                    norm: None,
                });
                break;
            }
        }
        tokens
    }
}
