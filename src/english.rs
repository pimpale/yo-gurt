use super::RuleSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use typed_arena::Arena;

// simple put utility method for arena
macro_rules! p {
    ($arena:expr, $string:expr) => {
        (&*$arena.alloc_str($string))
    };
}

macro_rules! rule {
    ($arena:expr, $hashmap:expr, $key:expr, [$($val:expr),* $(,)?]) => {
        $hashmap.insert(p!($arena, $key), vec![$( p!($arena, $val)),*])
    };
}

// Adapted from Spacy
pub fn ruleset<'vocab>(a: &'vocab mut Arena<u8>) -> RuleSet<'vocab> {
    // Prefixes
    let general_prefixes = HashSet::from_iter(vec![
        p!(a, "("),
        p!(a, ")"),
        p!(a, "$"),
        p!(a, "#"),
        p!(a, "."),
        p!(a, "'"),
        p!(a, "\""),
        p!(a, ".."),
        p!(a, "..."),
        p!(a, "&"),
        p!(a, "@"),
    ]);

    let general_suffixes = HashSet::from_iter(vec![
        p!(a, "("),
        p!(a, ")"),
        p!(a, "$"),
        p!(a, "#"),
        p!(a, "."),
        p!(a, "'"),
        p!(a, "\""),
        p!(a, ".."),
        p!(a, "..."),
        p!(a, "&"),
        p!(a, "@"),
    ]);

    let mut special = HashMap::new();
    rule!(a, special, "i'm", ["i", "am"]);
    rule!(a, special, "im", ["i", "am"]);
    rule!(a, special, "i'mma", ["i", "am", "going", "to"]);
    rule!(a, special, "imma", ["i", "am", "going", "to"]);

    for pronoun in ["i", "you", "he", "she", "it", "we", "they"].iter() {
        rule!(
            a,
            special,
            &format!("{}'ll", pronoun)[..],
            [pronoun, "will"]
        );
        rule!(
            a,
            special,
            &format!("{}'ll've", pronoun)[..],
            [pronoun, "will", "have"]
        );
        rule!(
            a,
            special,
            &format!("{}llve", pronoun)[..],
            [pronoun, "will", "have"]
        );
        rule!(
            a,
            special,
            &format!("{}'d", pronoun)[..],
            [pronoun, "would"]
        );
        rule!(a, special, &format!("{}d", pronoun)[..], [pronoun, "would"]);
        rule!(
            a,
            special,
            &format!("{}'d've", pronoun)[..],
            [pronoun, "would", "have"]
        );
        rule!(
            a,
            special,
            &format!("{}dve", pronoun)[..],
            [pronoun, "would", "have"]
        );
    }

    for pronoun in ["i", "you", "we", "they"].iter() {
        rule!(
            a,
            special,
            &format!("{}'ve", pronoun)[..],
            [pronoun, "have"]
        );
        rule!(a, special, &format!("{}ve", pronoun)[..], [pronoun, "have"]);
    }

    for pronoun in ["you", "we", "they"].iter() {
        rule!(a, special, &format!("{}'re", pronoun)[..], [pronoun, "are"]);
        // were, not we're
        if pronoun != &"we" {
            rule!(a, special, &format!("{}re", pronoun)[..], [pronoun, "are"]);
        }
    }

    // Posessives
    for pronoun in ["it", "he", "she"].iter() {
        rule!(a, special, &format!("{}'s", pronoun)[..], [pronoun, "'s"]);
        rule!(a, special, &format!("{}s", pronoun)[..], [pronoun, "'s"]);
        // "it" is special case
        if pronoun == &"it" {
            rule!(a, special, "its", ["it", "'s"]);
            rule!(a, special, "it's", ["it", "is"]);
        }
    }

    // W words, relative pronouns, and prepositions
    for word in [
        "who", "what", "when", "where", "why", "how", "there", "that",
    ]
    .iter()
    {
        // Possessives
        rule!(a, special, &format!("{}'s", word)[..], [word, "'s"]);
        rule!(a, special, &format!("{}s", word)[..], [word, "'s"]);
        // will
        rule!(a, special, &format!("{}'ll", word)[..], [word, "'ll"]);
        rule!(a, special, &format!("{}ll", word)[..], [word, "'ll"]);
        // have
        rule!(a, special, &format!("{}'ve", word)[..], [word, "have"]);
        rule!(a, special, &format!("{}ve", word)[..], [word, "have"]);
        // will have
        rule!(
            a,
            special,
            &format!("{}'ll've", word)[..],
            [word, "will", "have"]
        );
        rule!(
            a,
            special,
            &format!("{}llve", word)[..],
            [word, "will", "have"]
        );
        // would
        rule!(a, special, &format!("{}'d", word)[..], [word, "would"]);
        rule!(a, special, &format!("{}d", word)[..], [word, "would"]);
        // would have
        rule!(
            a,
            special,
            &format!("{}'d've", word)[..],
            [word, "would", "have"]
        );
        rule!(
            a,
            special,
            &format!("{}dve", word)[..],
            [word, "would", "have"]
        );
        // are
        rule!(a, special, &format!("{}'re", word)[..], [word, "are"]);
        rule!(a, special, &format!("{}re", word)[..], [word, "are"]);
    }

    RuleSet::new(general_prefixes, general_suffixes, special)
}
