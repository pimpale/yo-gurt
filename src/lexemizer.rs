use smol_str::SmolStr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

// simple put utility method for arena
macro_rules! sms {
    ($string:literal) => {{
        static_assertions::const_assert!($string.len() <= 22);
        SmolStr::new_inline_from_ascii($string.len(), $string)
    }};
}

macro_rules! add_rule {
    ($hashmap:expr, $key:expr, [$($val:expr),* $(,)?]) => {
        $hashmap.insert($key, vec![$($val),*])
    };
}

macro_rules! add_one_to_one_rule {
    ($hashmap:expr, $key:expr, $val:expr) => {
        add_rule!($hashmap, $key, [$val])
    };
}

macro_rules! add_exact_rule {
    ($hashmap:expr, $key:expr) => {
        add_one_to_one_rule!($hashmap, $key, $key)
    };
}

macro_rules! cat {
    ($str1:expr, $str2:expr) => {
        SmolStr::from([$str1, $str2].concat())
    };
}

#[derive(Debug)]
pub struct Lexeme {
    pub raw: String,
    pub norm: String,
}

pub struct RuleSet {
    // General Prefixes
    general_prefix: HashSet<SmolStr>, // Prefixes
    // General Suffixes
    general_suffix: HashSet<SmolStr>, // Suffixes (n't, 've, etc)
    special_expand: HashMap<SmolStr, Vec<SmolStr>>, // N.Y.. U.S., etc
}

impl RuleSet {
    // Adapted from Spacy
    pub fn english() -> RuleSet {
        // Prefixes
        let general_prefixes = HashSet::from_iter(vec![
            sms!(b"("),
            sms!(b")"),
            sms!(b"$"),
            sms!(b"#"),
            sms!(b"."),
            sms!(b"'"),
            sms!(b"\""),
            sms!(b".."),
            sms!(b"..."),
            sms!(b"&"),
            sms!(b"@"),
            sms!(b"?"),
        ]);

        let general_suffixes = HashSet::from_iter(vec![
            sms!(b"("),
            sms!(b")"),
            sms!(b"$"),
            sms!(b"#"),
            sms!(b"."),
            sms!(b"'"),
            sms!(b"\""),
            sms!(b".."),
            sms!(b"..."),
            sms!(b"&"),
            sms!(b"@"),
            sms!(b"?"),
        ]);

        let mut special = HashMap::new();
        add_rule!(special, sms!(b"i'm"), [sms!(b"i"), sms!(b"am")]);
        add_rule!(special, sms!(b"im"), [sms!(b"i"), sms!(b"am")]);
        add_rule!(
            special,
            sms!(b"i'mma"),
            [sms!(b"i"), sms!(b"am"), sms!(b"going"), sms!(b"to")]
        );
        add_rule!(
            special,
            sms!(b"imma"),
            [sms!(b"i"), sms!(b"am"), sms!(b"going"), sms!(b"to")]
        );

        for &pronoun in [
            sms!(b"i"),
            sms!(b"you"),
            sms!(b"he"),
            sms!(b"she"),
            sms!(b"it"),
            sms!(b"we"),
            sms!(b"they"),
        ]
        .iter()
        {
            add_rule!(
                special,
                cat!(pronoun, sms!(b"'ll")),
                [pronoun, sms!(b"will")]
            );
            add_rule!(
                special,
                cat!(pronoun.as_str(), "'ll've"),
                [pronoun, sms!(b"will"), sms!(b"have")]
            );
            add_rule!(
                special,
                cat!(pronoun, sms!(b"llve")),
                [pronoun, sms!(b"will"), sms!(b"have")]
            );
            add_rule!(
                special,
                cat!(pronoun, sms!(b"'d")),
                [pronoun, sms!(b"would")]
            );
            add_rule!(
                special,
                cat!(pronoun, sms!(b"d")),
                [pronoun, sms!(b"would")]
            );
            add_rule!(
                special,
                cat!(pronoun, sms!(b"'d've")),
                [pronoun, sms!(b"would"), sms!(b"have")]
            );
            add_rule!(
                special,
                cat!(pronoun, sms!(b"dve")),
                [pronoun, sms!(b"would"), sms!(b"have")]
            );
        }

        for &pronoun in [sms!(b"i"), sms!(b"you"), sms!(b"we"), sms!(b"they")].iter() {
            add_rule!(
                special,
                cat!(pronoun, sms!(b"'ve")),
                [pronoun, sms!(b"have")]
            );
            add_rule!(
                special,
                cat!(pronoun, sms!(b"ve")),
                [pronoun, sms!(b"have")]
            );
        }

        for &pronoun in [sms!(b"you"), sms!(b"we"), sms!(b"they")].iter() {
            add_rule!(
                special,
                cat!(pronoun, sms!(b"'re")),
                [pronoun, sms!(b"are")]
            );
            // were, not we're
            if pronoun != sms!(b"we") {
                add_rule!(special, cat!(sms!(b"re"), pronoun), [pronoun, sms!(b"are")]);
            }
        }

        // Posessives
        for &pronoun in [sms!(b"it"), sms!(b"he"), sms!(b"she")].iter() {
            add_rule!(special, cat!(pronoun, sms!(b"'s")), [pronoun, sms!(b"'s")]);
            add_rule!(special, cat!(pronoun, sms!(b"s")), [pronoun, sms!(b"'s")]);
            // sms!(b"it") is special case
            if pronoun == sms!(b"it") {
                add_rule!(special, sms!(b"its"), [sms!(b"it"), sms!(b"'s")]);
                add_rule!(special, sms!(b"it's"), [sms!(b"it"), sms!(b"is")]);
            }
        }

        // W words, relative pronouns, and prepositions
        for &word in [
            sms!(b"who"),
            sms!(b"what"),
            sms!(b"when"),
            sms!(b"where"),
            sms!(b"why"),
            sms!(b"how"),
            sms!(b"there"),
            sms!(b"that"),
        ]
        .iter()
        {
            // Possessives
            add_rule!(special, cat!(word, sms!(b"'s")), [word, sms!(b"'s")]);
            add_rule!(special, cat!(word, sms!(b"s")), [word, sms!(b"'s")]);
            // will
            add_rule!(special, cat!(word, sms!(b"'ll")), [word, sms!(b"'ll")]);
            add_rule!(special, cat!(word, sms!(b"ll")), [word, sms!(b"'ll")]);
            // have
            add_rule!(special, cat!(word, sms!(b"'ve")), [word, sms!(b"have")]);
            add_rule!(special, cat!(word, sms!(b"ve")), [word, sms!(b"have")]);
            // will have
            add_rule!(
                special,
                cat!(word, sms!(b"'ll've")),
                [word, sms!(b"will"), sms!(b"have")]
            );
            add_rule!(
                special,
                cat!(word, sms!(b"llve")),
                [word, sms!(b"will"), sms!(b"have")]
            );
            // would
            add_rule!(special, cat!(word, sms!(b"'d")), [word, sms!(b"would")]);
            add_rule!(special, cat!(word, sms!(b"d")), [word, sms!(b"would")]);
            // would have
            add_rule!(
                special,
                cat!(word, sms!(b"'d've")),
                [word, sms!(b"would"), sms!(b"have")]
            );
            add_rule!(
                special,
                cat!(word, sms!(b"dve")),
                [word, sms!(b"would"), sms!(b"have")]
            );
            // are
            add_rule!(special, cat!(word, sms!(b"'re")), [word, sms!(b"are")]);
            add_rule!(special, cat!(word, sms!(b"re")), [word, sms!(b"are")]);
        }

        for &word in [
            sms!(b"ca"),
            sms!(b"can"),
            sms!(b"could"),
            sms!(b"do"),
            sms!(b"does"),
            sms!(b"did"),
            sms!(b"had"),
            sms!(b"may"),
            sms!(b"might"),
            sms!(b"must"),
            sms!(b"need"),
            sms!(b"ought"),
            sms!(b"sha"),
            sms!(b"should"),
            sms!(b"wo"),
            sms!(b"would"),
        ]
        .iter()
        {
            add_rule!(special, cat!(word, sms!(b"n't")), [word, sms!(b"not")]);
            add_rule!(special, cat!(word, sms!(b"nt")), [word, sms!(b"not")]);
            add_rule!(
                special,
                cat!(word, sms!(b"n't've")),
                [word, sms!(b"not"), sms!(b"have")]
            );
            add_rule!(
                special,
                cat!(word, sms!(b"ntve")),
                [word, sms!(b"not"), sms!(b"have")]
            );
        }

        for &word in [
            sms!(b"could"),
            sms!(b"might"),
            sms!(b"must"),
            sms!(b"should"),
            sms!(b"would"),
        ]
        .iter()
        {
            add_rule!(special, cat!(word, sms!(b"'ve")), [word, sms!(b"have")]);
            add_rule!(special, cat!(word, sms!(b"ve")), [word, sms!(b"have")]);
        }

        for &word in [
            sms!(b"ai"),
            sms!(b"are"),
            sms!(b"is"),
            sms!(b"was"),
            sms!(b"were"),
            sms!(b"have"),
            sms!(b"has"),
            sms!(b"dare"),
        ]
        .iter()
        {
            add_rule!(special, cat!(word, sms!(b"n't")), [word, sms!(b"not")]);
            add_rule!(special, cat!(word, sms!(b"nt")), [word, sms!(b"not")]);
        }

        // other contractions/abbreviations
        add_rule!(special, sms!(b"y'all"), [sms!(b"you"), sms!(b"all")]);
        add_rule!(special, sms!(b"yall"), [sms!(b"you"), sms!(b"all")]);
        add_rule!(special, sms!(b"cannot"), [sms!(b"can"), sms!(b"not")]);
        add_rule!(special, sms!(b"gonna"), [sms!(b"going"), sms!(b"to")]);
        add_rule!(special, sms!(b"gotta"), [sms!(b"got"), sms!(b"to")]);
        add_rule!(special, sms!(b"let's"), [sms!(b"let"), sms!(b"us")]);
        add_rule!(special, sms!(b"lets"), [sms!(b"let"), sms!(b"us")]);
        add_rule!(special, sms!(b"'s"), [sms!(b"'s")]);
        add_rule!(special, sms!(b"and/or"), [sms!(b"and/or"),]);
        add_rule!(special, sms!(b"w/o"), [sms!(b"without")]);
        add_rule!(special, sms!(b"'re"), [sms!(b"are")]);
        add_rule!(special, sms!(b"'cause"), [sms!(b"because")]);
        add_rule!(special, sms!(b"'cos"), [sms!(b"because")]);
        add_rule!(special, sms!(b"'coz"), [sms!(b"because")]);
        add_rule!(special, sms!(b"'cuz"), [sms!(b"because")]);
        add_rule!(special, sms!(b"'bout"), [sms!(b"about")]);
        add_rule!(special, sms!(b"ma'am"), [sms!(b"madam")]);
        add_rule!(special, sms!(b"o'clock"), [sms!(b"o'clock")]);
        add_rule!(special, sms!(b"lovin'"), [sms!(b"loving")]);
        add_rule!(special, sms!(b"lovin"), [sms!(b"loving")]);
        add_rule!(special, sms!(b"havin'"), [sms!(b"having")]);
        add_rule!(special, sms!(b"havin"), [sms!(b"having")]);
        add_rule!(special, sms!(b"doin'"), [sms!(b"doing")]);
        add_rule!(special, sms!(b"doin"), [sms!(b"doing")]);
        add_rule!(special, sms!(b"goin'"), [sms!(b"going")]);
        add_rule!(special, sms!(b"goin"), [sms!(b"going")]);
        add_rule!(special, sms!(b"mt."), [sms!(b"mount")]);
        add_rule!(special, sms!(b"ak."), [sms!(b"alaska")]);
        add_rule!(special, sms!(b"ala."), [sms!(b"alabama")]);
        add_rule!(special, sms!(b"apr."), [sms!(b"april")]);
        add_rule!(special, sms!(b"ariz."), [sms!(b"arizona")]);
        add_rule!(special, sms!(b"ark."), [sms!(b"arkansas")]);
        add_rule!(special, sms!(b"aug."), [sms!(b"august")]);
        add_rule!(special, sms!(b"calif."), [sms!(b"california")]);
        add_rule!(special, sms!(b"colo."), [sms!(b"colorado")]);
        add_rule!(special, sms!(b"conn."), [sms!(b"connecticut")]);
        add_rule!(special, sms!(b"dec."), [sms!(b"december")]);
        add_rule!(special, sms!(b"del."), [sms!(b"delaware")]);
        add_rule!(special, sms!(b"feb."), [sms!(b"february")]);
        add_rule!(special, sms!(b"fla."), [sms!(b"florida")]);
        add_rule!(special, sms!(b"ga."), [sms!(b"georgia")]);
        add_rule!(special, sms!(b"ia."), [sms!(b"iowa")]);
        add_rule!(special, sms!(b"id."), [sms!(b"idaho")]);
        add_rule!(special, sms!(b"ill."), [sms!(b"illinois")]);
        add_rule!(special, sms!(b"ind."), [sms!(b"indiana")]);
        add_rule!(special, sms!(b"jan."), [sms!(b"january")]);
        add_rule!(special, sms!(b"jul."), [sms!(b"july")]);
        add_rule!(special, sms!(b"jun."), [sms!(b"june")]);
        add_rule!(special, sms!(b"kan."), [sms!(b"kansas")]);
        add_rule!(special, sms!(b"kans."), [sms!(b"kansas")]);
        add_rule!(special, sms!(b"ky."), [sms!(b"kentucky")]);
        add_rule!(special, sms!(b"la."), [sms!(b"louisiana")]);
        add_rule!(special, sms!(b"mar."), [sms!(b"march")]);
        add_rule!(special, sms!(b"mass."), [sms!(b"massachusetts")]);
        add_rule!(special, sms!(b"may."), [sms!(b"may")]);
        add_rule!(special, sms!(b"mich."), [sms!(b"michigan")]);
        add_rule!(special, sms!(b"minn."), [sms!(b"minnesota")]);
        add_rule!(special, sms!(b"miss."), [sms!(b"mississippi")]);
        add_rule!(special, sms!(b"n.c."), [sms!(b"north carolina")]);
        add_rule!(special, sms!(b"n.d."), [sms!(b"north dakota")]);
        add_rule!(special, sms!(b"n.h."), [sms!(b"new hampshire")]);
        add_rule!(special, sms!(b"n.j."), [sms!(b"new jersey")]);
        add_rule!(special, sms!(b"n.m."), [sms!(b"new mexico")]);
        add_rule!(special, sms!(b"n.y."), [sms!(b"new york")]);
        add_rule!(special, sms!(b"neb."), [sms!(b"nebraska")]);
        add_rule!(special, sms!(b"nebr."), [sms!(b"nebraska")]);
        add_rule!(special, sms!(b"nev."), [sms!(b"nevada")]);
        add_rule!(special, sms!(b"nov."), [sms!(b"november")]);
        add_rule!(special, sms!(b"oct."), [sms!(b"october")]);
        add_rule!(special, sms!(b"okla."), [sms!(b"oklahoma")]);
        add_rule!(special, sms!(b"ore."), [sms!(b"oregon")]);
        add_rule!(special, sms!(b"pa."), [sms!(b"pennsylvania")]);
        add_rule!(special, sms!(b"s.c."), [sms!(b"south carolina")]);
        add_rule!(special, sms!(b"sep."), [sms!(b"september")]);
        add_rule!(special, sms!(b"sept."), [sms!(b"september")]);
        add_rule!(special, sms!(b"tenn."), [sms!(b"tennessee")]);
        add_rule!(special, sms!(b"va."), [sms!(b"virginia")]);
        add_rule!(special, sms!(b"wash."), [sms!(b"washington")]);
        add_rule!(special, sms!(b"wis."), [sms!(b"wisconsin")]);

        // yet more abbreviations
        add_exact_rule!(special, sms!(b"'d"));
        add_exact_rule!(special, sms!(b"a.m."));
        add_exact_rule!(special, sms!(b"adm."));
        add_exact_rule!(special, sms!(b"bros."));
        add_exact_rule!(special, sms!(b"co."));
        add_exact_rule!(special, sms!(b"corp."));
        add_exact_rule!(special, sms!(b"d.c."));
        add_exact_rule!(special, sms!(b"dr."));
        add_exact_rule!(special, sms!(b"e.g."));
        add_exact_rule!(special, sms!(b"gen."));
        add_exact_rule!(special, sms!(b"gov."));
        add_exact_rule!(special, sms!(b"i.e."));
        add_exact_rule!(special, sms!(b"inc."));
        add_exact_rule!(special, sms!(b"jr."));
        add_exact_rule!(special, sms!(b"ltd."));
        add_exact_rule!(special, sms!(b"md."));
        add_exact_rule!(special, sms!(b"messrs."));
        add_exact_rule!(special, sms!(b"mo."));
        add_exact_rule!(special, sms!(b"mont."));
        add_exact_rule!(special, sms!(b"mr."));
        add_exact_rule!(special, sms!(b"mrs."));
        add_exact_rule!(special, sms!(b"ms."));
        add_exact_rule!(special, sms!(b"p.m."));
        add_exact_rule!(special, sms!(b"ph.d."));
        add_exact_rule!(special, sms!(b"prof."));
        add_exact_rule!(special, sms!(b"rep."));
        add_exact_rule!(special, sms!(b"rev."));
        add_exact_rule!(special, sms!(b"sen."));
        add_exact_rule!(special, sms!(b"st."));
        add_exact_rule!(special, sms!(b"vs."));
        add_exact_rule!(special, sms!(b"v.s."));

        RuleSet {
            general_prefix: general_prefixes,
            general_suffix: general_suffixes,
            special_expand: special,
        }
    }

    // If there is an exact match between this string and a special expand,
    // We create a set of lexemes with norms and the text
    pub fn special_expand<'doc>(&self, string: &'doc str) -> Option<Vec<Lexeme>> {
        let ret = self.special_expand.get(&string);
        if let Some(vs) = ret {
            Some(
                vs.iter()
                    .map(|norm| Lexeme {
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
    // Returns A remainder, and a prefix lexeme
    pub fn general_prefix_remainder<'doc>(
        &self,
        string: &'doc str,
    ) -> Option<(Lexeme, &String)> {
        for i in (1..string.len()).rev() {
            if let Some(prefix_norm) = self.general_prefix.get(&string[..i]) {
                return Some((
                    Lexeme {
                        string: &string[..i].to_string(),
                        norm: prefix_norm.to_string(),
                    },
                    &string[i..],
                ));
            }
        }
        None
    }

    // Matches the longest suffix
    pub fn general_suffix_remainder<'doc>(&self, string: &'doc str) -> Option<(Lexeme, &'doc str)> {
        for i in 1..string.len() {
            if let Some(suffix_norm) = self.general_suffix.get(&string[i..]) {
                return Some((
                    Lexeme {
                        string: &string[i..],
                        norm: Some(suffix_norm),
                    },
                    &string[..i],
                ));
            }
        }
        None
    }

    // Lexemize LOWERCASE string
    // Uses spacy algorithm
    pub fn lexemize(&self, string: String) -> Vec<Lexeme> {
        let mut lexemes = Vec::new();

        for s in string.split_whitespace() {
            let mut substr = s;
            loop {
                if let Some(mut tokvec) = self.special_expand(&substr) {
                    lexemes.append(&mut tokvec);
                    // this will cause us to start viewing the next substr
                    break;
                } else if let Some((lexeme, remainder)) = self.general_prefix_remainder(&substr) {
                    lexemes.push(lexeme);
                    substr = remainder;
                    continue;
                } else if let Some((lexeme, remainder)) = self.general_suffix_remainder(&substr) {
                    lexemes.push(lexeme);
                    substr = remainder;
                    continue;
                }
                // If we can't do anything with it, push lexeme
                lexemes.push(Lexeme {
                    string: substr,
                    norm: None,
                });
                break;
            }
        }
        lexemes
    }
}
