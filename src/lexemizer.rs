use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use unidecode::unidecode;

// Convert tiny string into smolstr
macro_rules! sv {
    ($string:expr) => {{
        $string.to_vec()
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
        ([$str1, $str2].concat())
    };
}

#[derive(Debug)]
pub struct Lexeme {
    pub value: Vec<u8>,
}

pub struct RuleSet {
    // General Prefixes
    general_prefix: HashSet<Vec<u8>>, // Prefixes
    // General Suffixes
    general_suffix: HashSet<Vec<u8>>, // Suffixes (n't, 've, etc)
    special_expand: HashMap<Vec<u8>, Vec<Vec<u8>>>, // N.Y.. U.S., etc
}

impl RuleSet {
    // Adapted from Spacy
    pub fn english() -> RuleSet {
        // Prefixes
        let general_prefixes = HashSet::from_iter(vec![
            sv!(b"("),
            sv!(b")"),
            sv!(b"$"),
            sv!(b"#"),
            sv!(b"."),
            sv!(b"'"),
            sv!(b"\""),
            sv!(b".."),
            sv!(b"..."),
            sv!(b"&"),
            sv!(b"@"),
            sv!(b"?"),
        ]);

        let general_suffixes = HashSet::from_iter(vec![
            sv!(b"("),
            sv!(b")"),
            sv!(b"$"),
            sv!(b"#"),
            sv!(b"."),
            sv!(b"'"),
            sv!(b"\""),
            sv!(b".."),
            sv!(b"..."),
            sv!(b"&"),
            sv!(b"@"),
            sv!(b"?"),
        ]);

        let mut special = HashMap::new();
        add_rule!(special, sv!(b"i'm"), [sv!(b"i"), sv!(b"am")]);
        add_rule!(special, sv!(b"im"), [sv!(b"i"), sv!(b"am")]);
        add_rule!(
            special,
            sv!(b"i'mma"),
            [sv!(b"i"), sv!(b"am"), sv!(b"going"), sv!(b"to")]
        );
        add_rule!(
            special,
            sv!(b"imma"),
            [sv!(b"i"), sv!(b"am"), sv!(b"going"), sv!(b"to")]
        );

        for pronoun in [
            sv!(b"i"),
            sv!(b"you"),
            sv!(b"he"),
            sv!(b"she"),
            sv!(b"it"),
            sv!(b"we"),
            sv!(b"they"),
        ]
        .iter()
        {
            add_rule!(
                special,
                cat!(pronoun.clone(), sv!(b"'ll")),
                [pronoun.clone(), sv!(b"will")]
            );
            add_rule!(
                special,
                cat!(pronoun.clone(), sv!(b"'ll've")),
                [pronoun.clone(), sv!(b"will"), sv!(b"have")]
            );
            add_rule!(
                special,
                cat!(pronoun.clone(), sv!(b"llve")),
                [pronoun.clone(), sv!(b"will"), sv!(b"have")]
            );
            add_rule!(
                special,
                cat!(pronoun.clone(), sv!(b"'d")),
                [pronoun.clone(), sv!(b"would")]
            );
            add_rule!(
                special,
                cat!(pronoun.clone(), sv!(b"d")),
                [pronoun.clone(), sv!(b"would")]
            );
            add_rule!(
                special,
                cat!(pronoun.clone(), sv!(b"'d've")),
                [pronoun.clone(), sv!(b"would"), sv!(b"have")]
            );
            add_rule!(
                special,
                cat!(pronoun.clone(), sv!(b"dve")),
                [pronoun.clone(), sv!(b"would"), sv!(b"have")]
            );
        }

        for pronoun in [sv!(b"i"), sv!(b"you"), sv!(b"we"), sv!(b"they")].iter() {
            add_rule!(
                special,
                cat!(pronoun.clone(), sv!(b"'ve")),
                [pronoun.clone(), sv!(b"have")]
            );
            add_rule!(
                special,
                cat!(pronoun.clone(), sv!(b"ve")),
                [pronoun.clone(), sv!(b"have")]
            );
        }

        for pronoun in [sv!(b"you"), sv!(b"we"), sv!(b"they")].iter() {
            add_rule!(
                special,
                cat!(pronoun.clone(), sv!(b"'re")),
                [pronoun.clone(), sv!(b"are")]
            );
            // were, not we're
            if pronoun != b"we" {
                add_rule!(
                    special,
                    cat!(sv!(b"re"), pronoun.clone()),
                    [pronoun.clone(), sv!(b"are")]
                );
            }
        }

        // Posessives
        for pronoun in [sv!(b"it"), sv!(b"he"), sv!(b"she")].iter() {
            add_rule!(
                special,
                cat!(pronoun.clone(), sv!(b"'s")),
                [pronoun.clone(), sv!(b"'s")]
            );
            add_rule!(
                special,
                cat!(pronoun.clone(), sv!(b"s")),
                [pronoun.clone(), sv!(b"'s")]
            );
            // sv!(b"it") is special case
            if pronoun == b"it" {
                add_rule!(special, sv!(b"its"), [sv!(b"it"), sv!(b"'s")]);
                add_rule!(special, sv!(b"it's"), [sv!(b"it"), sv!(b"is")]);
            }
        }

        // W words, relative pronouns, and prepositions
        for word in [
            sv!(b"who"),
            sv!(b"what"),
            sv!(b"when"),
            sv!(b"where"),
            sv!(b"why"),
            sv!(b"how"),
            sv!(b"there"),
            sv!(b"that"),
        ]
        .iter()
        {
            // Possessives
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"'s")),
                [word.clone(), sv!(b"'s")]
            );
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"s")),
                [word.clone(), sv!(b"'s")]
            );
            // will
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"'ll")),
                [word.clone(), sv!(b"'ll")]
            );
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"ll")),
                [word.clone(), sv!(b"'ll")]
            );
            // have
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"'ve")),
                [word.clone(), sv!(b"have")]
            );
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"ve")),
                [word.clone(), sv!(b"have")]
            );
            // will have
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"'ll've")),
                [word.clone(), sv!(b"will"), sv!(b"have")]
            );
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"llve")),
                [word.clone(), sv!(b"will"), sv!(b"have")]
            );
            // would
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"'d")),
                [word.clone(), sv!(b"would")]
            );
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"d")),
                [word.clone(), sv!(b"would")]
            );
            // would have
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"'d've")),
                [word.clone(), sv!(b"would"), sv!(b"have")]
            );
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"dve")),
                [word.clone(), sv!(b"would"), sv!(b"have")]
            );
            // are
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"'re")),
                [word.clone(), sv!(b"are")]
            );
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"re")),
                [word.clone(), sv!(b"are")]
            );
        }

        for word in [
            sv!(b"ca"),
            sv!(b"can"),
            sv!(b"could"),
            sv!(b"do"),
            sv!(b"does"),
            sv!(b"did"),
            sv!(b"had"),
            sv!(b"may"),
            sv!(b"might"),
            sv!(b"must"),
            sv!(b"need"),
            sv!(b"ought"),
            sv!(b"sha"),
            sv!(b"should"),
            sv!(b"wo"),
            sv!(b"would"),
        ]
        .iter()
        {
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"n't")),
                [word.clone(), sv!(b"not")]
            );
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"nt")),
                [word.clone(), sv!(b"not")]
            );
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"n't've")),
                [word.clone(), sv!(b"not"), sv!(b"have")]
            );
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"ntve")),
                [word.clone(), sv!(b"not"), sv!(b"have")]
            );
        }

        for word in [
            sv!(b"could"),
            sv!(b"might"),
            sv!(b"must"),
            sv!(b"should"),
            sv!(b"would"),
        ]
        .iter()
        {
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"'ve")),
                [word.clone(), sv!(b"have")]
            );
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"ve")),
                [word.clone(), sv!(b"have")]
            );
        }

        for word in [
            sv!(b"ai"),
            sv!(b"are"),
            sv!(b"is"),
            sv!(b"was"),
            sv!(b"were"),
            sv!(b"have"),
            sv!(b"has"),
            sv!(b"dare"),
        ]
        .iter()
        {
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"n't")),
                [word.clone(), sv!(b"not")]
            );
            add_rule!(
                special,
                cat!(word.clone(), sv!(b"nt")),
                [word.clone(), sv!(b"not")]
            );
        }

        // other contractions/abbreviations
        add_rule!(special, sv!(b"y'all"), [sv!(b"you"), sv!(b"all")]);
        add_rule!(special, sv!(b"yall"), [sv!(b"you"), sv!(b"all")]);
        add_rule!(special, sv!(b"cannot"), [sv!(b"can"), sv!(b"not")]);
        add_rule!(special, sv!(b"gonna"), [sv!(b"going"), sv!(b"to")]);
        add_rule!(special, sv!(b"gotta"), [sv!(b"got"), sv!(b"to")]);
        add_rule!(special, sv!(b"let's"), [sv!(b"let"), sv!(b"us")]);
        add_rule!(special, sv!(b"lets"), [sv!(b"let"), sv!(b"us")]);
        add_rule!(special, sv!(b"'s"), [sv!(b"'s")]);
        add_rule!(special, sv!(b"and/or"), [sv!(b"and/or"),]);
        add_rule!(special, sv!(b"w/o"), [sv!(b"without")]);
        add_rule!(special, sv!(b"'re"), [sv!(b"are")]);
        add_rule!(special, sv!(b"'cause"), [sv!(b"because")]);
        add_rule!(special, sv!(b"'cos"), [sv!(b"because")]);
        add_rule!(special, sv!(b"'coz"), [sv!(b"because")]);
        add_rule!(special, sv!(b"'cuz"), [sv!(b"because")]);
        add_rule!(special, sv!(b"'bout"), [sv!(b"about")]);
        add_rule!(special, sv!(b"ma'am"), [sv!(b"madam")]);
        add_rule!(special, sv!(b"o'clock"), [sv!(b"o'clock")]);
        add_rule!(special, sv!(b"lovin'"), [sv!(b"loving")]);
        add_rule!(special, sv!(b"lovin"), [sv!(b"loving")]);
        add_rule!(special, sv!(b"havin'"), [sv!(b"having")]);
        add_rule!(special, sv!(b"havin"), [sv!(b"having")]);
        add_rule!(special, sv!(b"doin'"), [sv!(b"doing")]);
        add_rule!(special, sv!(b"doin"), [sv!(b"doing")]);
        add_rule!(special, sv!(b"goin'"), [sv!(b"going")]);
        add_rule!(special, sv!(b"goin"), [sv!(b"going")]);
        add_rule!(special, sv!(b"mt."), [sv!(b"mount")]);
        add_rule!(special, sv!(b"ak."), [sv!(b"alaska")]);
        add_rule!(special, sv!(b"ala."), [sv!(b"alabama")]);
        add_rule!(special, sv!(b"apr."), [sv!(b"april")]);
        add_rule!(special, sv!(b"ariz."), [sv!(b"arizona")]);
        add_rule!(special, sv!(b"ark."), [sv!(b"arkansas")]);
        add_rule!(special, sv!(b"aug."), [sv!(b"august")]);
        add_rule!(special, sv!(b"calif."), [sv!(b"california")]);
        add_rule!(special, sv!(b"colo."), [sv!(b"colorado")]);
        add_rule!(special, sv!(b"conn."), [sv!(b"connecticut")]);
        add_rule!(special, sv!(b"dec."), [sv!(b"december")]);
        add_rule!(special, sv!(b"del."), [sv!(b"delaware")]);
        add_rule!(special, sv!(b"feb."), [sv!(b"february")]);
        add_rule!(special, sv!(b"fla."), [sv!(b"florida")]);
        add_rule!(special, sv!(b"ga."), [sv!(b"georgia")]);
        add_rule!(special, sv!(b"ia."), [sv!(b"iowa")]);
        add_rule!(special, sv!(b"id."), [sv!(b"idaho")]);
        add_rule!(special, sv!(b"ill."), [sv!(b"illinois")]);
        add_rule!(special, sv!(b"ind."), [sv!(b"indiana")]);
        add_rule!(special, sv!(b"jan."), [sv!(b"january")]);
        add_rule!(special, sv!(b"jul."), [sv!(b"july")]);
        add_rule!(special, sv!(b"jun."), [sv!(b"june")]);
        add_rule!(special, sv!(b"kan."), [sv!(b"kansas")]);
        add_rule!(special, sv!(b"kans."), [sv!(b"kansas")]);
        add_rule!(special, sv!(b"ky."), [sv!(b"kentucky")]);
        add_rule!(special, sv!(b"la."), [sv!(b"louisiana")]);
        add_rule!(special, sv!(b"mar."), [sv!(b"march")]);
        add_rule!(special, sv!(b"mass."), [sv!(b"massachusetts")]);
        add_rule!(special, sv!(b"may."), [sv!(b"may")]);
        add_rule!(special, sv!(b"mich."), [sv!(b"michigan")]);
        add_rule!(special, sv!(b"minn."), [sv!(b"minnesota")]);
        add_rule!(special, sv!(b"miss."), [sv!(b"mississippi")]);
        add_rule!(special, sv!(b"n.c."), [sv!(b"north carolina")]);
        add_rule!(special, sv!(b"n.d."), [sv!(b"north dakota")]);
        add_rule!(special, sv!(b"n.h."), [sv!(b"new hampshire")]);
        add_rule!(special, sv!(b"n.j."), [sv!(b"new jersey")]);
        add_rule!(special, sv!(b"n.m."), [sv!(b"new mexico")]);
        add_rule!(special, sv!(b"n.y."), [sv!(b"new york")]);
        add_rule!(special, sv!(b"neb."), [sv!(b"nebraska")]);
        add_rule!(special, sv!(b"nebr."), [sv!(b"nebraska")]);
        add_rule!(special, sv!(b"nev."), [sv!(b"nevada")]);
        add_rule!(special, sv!(b"nov."), [sv!(b"november")]);
        add_rule!(special, sv!(b"oct."), [sv!(b"october")]);
        add_rule!(special, sv!(b"okla."), [sv!(b"oklahoma")]);
        add_rule!(special, sv!(b"ore."), [sv!(b"oregon")]);
        add_rule!(special, sv!(b"pa."), [sv!(b"pennsylvania")]);
        add_rule!(special, sv!(b"s.c."), [sv!(b"south carolina")]);
        add_rule!(special, sv!(b"sep."), [sv!(b"september")]);
        add_rule!(special, sv!(b"sept."), [sv!(b"september")]);
        add_rule!(special, sv!(b"tenn."), [sv!(b"tennessee")]);
        add_rule!(special, sv!(b"va."), [sv!(b"virginia")]);
        add_rule!(special, sv!(b"wash."), [sv!(b"washington")]);
        add_rule!(special, sv!(b"wis."), [sv!(b"wisconsin")]);

        // yet more abbreviations
        add_exact_rule!(special, sv!(b"'d"));
        add_exact_rule!(special, sv!(b"a.m."));
        add_exact_rule!(special, sv!(b"adm."));
        add_exact_rule!(special, sv!(b"bros."));
        add_exact_rule!(special, sv!(b"co."));
        add_exact_rule!(special, sv!(b"corp."));
        add_exact_rule!(special, sv!(b"d.c."));
        add_exact_rule!(special, sv!(b"dr."));
        add_exact_rule!(special, sv!(b"e.g."));
        add_exact_rule!(special, sv!(b"gen."));
        add_exact_rule!(special, sv!(b"gov."));
        add_exact_rule!(special, sv!(b"i.e."));
        add_exact_rule!(special, sv!(b"inc."));
        add_exact_rule!(special, sv!(b"jr."));
        add_exact_rule!(special, sv!(b"ltd."));
        add_exact_rule!(special, sv!(b"md."));
        add_exact_rule!(special, sv!(b"messrs."));
        add_exact_rule!(special, sv!(b"mo."));
        add_exact_rule!(special, sv!(b"mont."));
        add_exact_rule!(special, sv!(b"mr."));
        add_exact_rule!(special, sv!(b"mrs."));
        add_exact_rule!(special, sv!(b"ms."));
        add_exact_rule!(special, sv!(b"p.m."));
        add_exact_rule!(special, sv!(b"ph.d."));
        add_exact_rule!(special, sv!(b"prof."));
        add_exact_rule!(special, sv!(b"rep."));
        add_exact_rule!(special, sv!(b"rev."));
        add_exact_rule!(special, sv!(b"sen."));
        add_exact_rule!(special, sv!(b"st."));
        add_exact_rule!(special, sv!(b"vs."));
        add_exact_rule!(special, sv!(b"v.s."));

        // british
        add_rule!(special, sv!(b"accessorise"), [sv!(b"accessorize")]);
        add_rule!(special, sv!(b"accessorised"), [sv!(b"accessorized")]);
        add_rule!(special, sv!(b"accessorises"), [sv!(b"accessorizes")]);
        add_rule!(special, sv!(b"accessorising"), [sv!(b"accessorizing")]);
        add_rule!(special, sv!(b"acclimatisation"), [sv!(b"acclimatization")]);
        add_rule!(special, sv!(b"acclimatise"), [sv!(b"acclimatize")]);
        add_rule!(special, sv!(b"acclimatised"), [sv!(b"acclimatized")]);
        add_rule!(special, sv!(b"acclimatises"), [sv!(b"acclimatizes")]);
        add_rule!(special, sv!(b"acclimatising"), [sv!(b"acclimatizing")]);
        add_rule!(special, sv!(b"accoutrements"), [sv!(b"accouterments")]);
        add_rule!(special, sv!(b"aeon"), [sv!(b"eon")]);
        add_rule!(special, sv!(b"aeons"), [sv!(b"eons")]);
        add_rule!(special, sv!(b"aerogramme"), [sv!(b"aerogram")]);
        add_rule!(special, sv!(b"aerogrammes"), [sv!(b"aerograms")]);
        add_rule!(special, sv!(b"aeroplane"), [sv!(b"airplane")]);
        add_rule!(special, sv!(b"aeroplanes "), [sv!(b"airplanes ")]);
        add_rule!(special, sv!(b"aesthete"), [sv!(b"esthete")]);
        add_rule!(special, sv!(b"aesthetes"), [sv!(b"esthetes")]);
        add_rule!(special, sv!(b"aesthetic"), [sv!(b"esthetic")]);
        add_rule!(special, sv!(b"aesthetically"), [sv!(b"esthetically")]);
        add_rule!(special, sv!(b"aesthetics"), [sv!(b"esthetics")]);
        add_rule!(special, sv!(b"aetiology"), [sv!(b"etiology")]);
        add_rule!(special, sv!(b"ageing"), [sv!(b"aging")]);
        add_rule!(special, sv!(b"aggrandisement"), [sv!(b"aggrandizement")]);
        add_rule!(special, sv!(b"agonise"), [sv!(b"agonize")]);
        add_rule!(special, sv!(b"agonised"), [sv!(b"agonized")]);
        add_rule!(special, sv!(b"agonises"), [sv!(b"agonizes")]);
        add_rule!(special, sv!(b"agonising"), [sv!(b"agonizing")]);
        add_rule!(special, sv!(b"agonisingly"), [sv!(b"agonizingly")]);
        add_rule!(special, sv!(b"almanack"), [sv!(b"almanac")]);
        add_rule!(special, sv!(b"almanacks"), [sv!(b"almanacs")]);
        add_rule!(special, sv!(b"aluminium"), [sv!(b"aluminum")]);
        add_rule!(special, sv!(b"amortisable"), [sv!(b"amortizable")]);
        add_rule!(special, sv!(b"amortisation"), [sv!(b"amortization")]);
        add_rule!(special, sv!(b"amortisations"), [sv!(b"amortizations")]);
        add_rule!(special, sv!(b"amortise"), [sv!(b"amortize")]);
        add_rule!(special, sv!(b"amortised"), [sv!(b"amortized")]);
        add_rule!(special, sv!(b"amortises"), [sv!(b"amortizes")]);
        add_rule!(special, sv!(b"amortising"), [sv!(b"amortizing")]);
        add_rule!(special, sv!(b"amphitheatre"), [sv!(b"amphitheater")]);
        add_rule!(special, sv!(b"amphitheatres"), [sv!(b"amphitheaters")]);
        add_rule!(special, sv!(b"anaemia"), [sv!(b"anemia")]);
        add_rule!(special, sv!(b"anaemic"), [sv!(b"anemic")]);
        add_rule!(special, sv!(b"anaesthesia"), [sv!(b"anesthesia")]);
        add_rule!(special, sv!(b"anaesthetic"), [sv!(b"anesthetic")]);
        add_rule!(special, sv!(b"anaesthetics"), [sv!(b"anesthetics")]);
        add_rule!(special, sv!(b"anaesthetise"), [sv!(b"anesthetize")]);
        add_rule!(special, sv!(b"anaesthetised"), [sv!(b"anesthetized")]);
        add_rule!(special, sv!(b"anaesthetises"), [sv!(b"anesthetizes")]);
        add_rule!(special, sv!(b"anaesthetising"), [sv!(b"anesthetizing")]);
        add_rule!(special, sv!(b"anaesthetist"), [sv!(b"anesthetist")]);
        add_rule!(special, sv!(b"anaesthetists"), [sv!(b"anesthetists")]);
        add_rule!(special, sv!(b"anaesthetize"), [sv!(b"anesthetize")]);
        add_rule!(special, sv!(b"anaesthetized"), [sv!(b"anesthetized")]);
        add_rule!(special, sv!(b"anaesthetizes"), [sv!(b"anesthetizes")]);
        add_rule!(special, sv!(b"anaesthetizing"), [sv!(b"anesthetizing")]);
        add_rule!(special, sv!(b"analogue"), [sv!(b"analog")]);
        add_rule!(special, sv!(b"analogues"), [sv!(b"analogs")]);
        add_rule!(special, sv!(b"analyse"), [sv!(b"analyze")]);
        add_rule!(special, sv!(b"analysed"), [sv!(b"analyzed")]);
        add_rule!(special, sv!(b"analyses"), [sv!(b"analyzes")]);
        add_rule!(special, sv!(b"analysing"), [sv!(b"analyzing")]);
        add_rule!(special, sv!(b"anglicise"), [sv!(b"anglicize")]);
        add_rule!(special, sv!(b"anglicised"), [sv!(b"anglicized")]);
        add_rule!(special, sv!(b"anglicises"), [sv!(b"anglicizes")]);
        add_rule!(special, sv!(b"anglicising"), [sv!(b"anglicizing")]);
        add_rule!(special, sv!(b"annualised"), [sv!(b"annualized")]);
        add_rule!(special, sv!(b"antagonise"), [sv!(b"antagonize")]);
        add_rule!(special, sv!(b"antagonised"), [sv!(b"antagonized")]);
        add_rule!(special, sv!(b"antagonises"), [sv!(b"antagonizes")]);
        add_rule!(special, sv!(b"antagonising"), [sv!(b"antagonizing")]);
        add_rule!(special, sv!(b"apologise"), [sv!(b"apologize")]);
        add_rule!(special, sv!(b"apologised"), [sv!(b"apologized")]);
        add_rule!(special, sv!(b"apologises"), [sv!(b"apologizes")]);
        add_rule!(special, sv!(b"apologising"), [sv!(b"apologizing")]);
        add_rule!(special, sv!(b"appal"), [sv!(b"appall")]);
        add_rule!(special, sv!(b"appals"), [sv!(b"appalls")]);
        add_rule!(special, sv!(b"appetiser"), [sv!(b"appetizer")]);
        add_rule!(special, sv!(b"appetisers"), [sv!(b"appetizers")]);
        add_rule!(special, sv!(b"appetising"), [sv!(b"appetizing")]);
        add_rule!(special, sv!(b"appetisingly"), [sv!(b"appetizingly")]);
        add_rule!(special, sv!(b"arbour"), [sv!(b"arbor")]);
        add_rule!(special, sv!(b"arbours"), [sv!(b"arbors")]);
        add_rule!(special, sv!(b"archaeological"), [sv!(b"archeological")]);
        add_rule!(special, sv!(b"archaeologically"), [sv!(b"archeologically")]);
        add_rule!(special, sv!(b"archaeologist"), [sv!(b"archeologist")]);
        add_rule!(special, sv!(b"archaeologists"), [sv!(b"archeologists")]);
        add_rule!(special, sv!(b"archaeology"), [sv!(b"archeology")]);
        add_rule!(special, sv!(b"ardour"), [sv!(b"ardor")]);
        add_rule!(special, sv!(b"armour"), [sv!(b"armor")]);
        add_rule!(special, sv!(b"armoured"), [sv!(b"armored")]);
        add_rule!(special, sv!(b"armourer"), [sv!(b"armorer")]);
        add_rule!(special, sv!(b"armourers"), [sv!(b"armorers")]);
        add_rule!(special, sv!(b"armouries"), [sv!(b"armories")]);
        add_rule!(special, sv!(b"armoury"), [sv!(b"armory")]);
        add_rule!(special, sv!(b"artefact"), [sv!(b"artifact")]);
        add_rule!(special, sv!(b"artefacts"), [sv!(b"artifacts")]);
        add_rule!(special, sv!(b"authorise"), [sv!(b"authorize")]);
        add_rule!(special, sv!(b"authorised"), [sv!(b"authorized")]);
        add_rule!(special, sv!(b"authorises"), [sv!(b"authorizes")]);
        add_rule!(special, sv!(b"authorising"), [sv!(b"authorizing")]);
        add_rule!(special, sv!(b"axe"), [sv!(b"ax")]);
        add_rule!(special, sv!(b"backpedalled"), [sv!(b"backpedaled")]);
        add_rule!(special, sv!(b"backpedalling"), [sv!(b"backpedaling")]);
        add_rule!(special, sv!(b"bannister"), [sv!(b"banister")]);
        add_rule!(special, sv!(b"bannisters"), [sv!(b"banisters")]);
        add_rule!(special, sv!(b"baptise"), [sv!(b"baptize")]);
        add_rule!(special, sv!(b"baptised"), [sv!(b"baptized")]);
        add_rule!(special, sv!(b"baptises"), [sv!(b"baptizes")]);
        add_rule!(special, sv!(b"baptising"), [sv!(b"baptizing")]);
        add_rule!(special, sv!(b"bastardise"), [sv!(b"bastardize")]);
        add_rule!(special, sv!(b"bastardised"), [sv!(b"bastardized")]);
        add_rule!(special, sv!(b"bastardises"), [sv!(b"bastardizes")]);
        add_rule!(special, sv!(b"bastardising"), [sv!(b"bastardizing")]);
        add_rule!(special, sv!(b"battleaxe"), [sv!(b"battleax")]);
        add_rule!(special, sv!(b"baulk"), [sv!(b"balk")]);
        add_rule!(special, sv!(b"baulked"), [sv!(b"balked")]);
        add_rule!(special, sv!(b"baulking"), [sv!(b"balking")]);
        add_rule!(special, sv!(b"baulks"), [sv!(b"balks")]);
        add_rule!(special, sv!(b"bedevilled"), [sv!(b"bedeviled")]);
        add_rule!(special, sv!(b"bedevilling"), [sv!(b"bedeviling")]);
        add_rule!(special, sv!(b"behaviour"), [sv!(b"behavior")]);
        add_rule!(special, sv!(b"behavioural"), [sv!(b"behavioral")]);
        add_rule!(special, sv!(b"behaviourism"), [sv!(b"behaviorism")]);
        add_rule!(special, sv!(b"behaviourist"), [sv!(b"behaviorist")]);
        add_rule!(special, sv!(b"behaviourists"), [sv!(b"behaviorists")]);
        add_rule!(special, sv!(b"behaviours"), [sv!(b"behaviors")]);
        add_rule!(special, sv!(b"behove"), [sv!(b"behoove")]);
        add_rule!(special, sv!(b"behoved"), [sv!(b"behooved")]);
        add_rule!(special, sv!(b"behoves"), [sv!(b"behooves")]);
        add_rule!(special, sv!(b"bejewelled"), [sv!(b"bejeweled")]);
        add_rule!(special, sv!(b"belabour"), [sv!(b"belabor")]);
        add_rule!(special, sv!(b"belaboured"), [sv!(b"belabored")]);
        add_rule!(special, sv!(b"belabouring"), [sv!(b"belaboring")]);
        add_rule!(special, sv!(b"belabours"), [sv!(b"belabors")]);
        add_rule!(special, sv!(b"bevelled"), [sv!(b"beveled")]);
        add_rule!(special, sv!(b"bevvies"), [sv!(b"bevies")]);
        add_rule!(special, sv!(b"bevvy"), [sv!(b"bevy")]);
        add_rule!(special, sv!(b"biassed"), [sv!(b"biased")]);
        add_rule!(special, sv!(b"biassing"), [sv!(b"biasing")]);
        add_rule!(special, sv!(b"bingeing"), [sv!(b"binging")]);
        add_rule!(special, sv!(b"bougainvillaea"), [sv!(b"bougainvillea")]);
        add_rule!(special, sv!(b"bougainvillaeas"), [sv!(b"bougainvilleas")]);
        add_rule!(special, sv!(b"bowdlerise"), [sv!(b"bowdlerize")]);
        add_rule!(special, sv!(b"bowdlerised"), [sv!(b"bowdlerized")]);
        add_rule!(special, sv!(b"bowdlerises"), [sv!(b"bowdlerizes")]);
        add_rule!(special, sv!(b"bowdlerising"), [sv!(b"bowdlerizing")]);
        add_rule!(special, sv!(b"breathalyse"), [sv!(b"breathalyze")]);
        add_rule!(special, sv!(b"breathalysed"), [sv!(b"breathalyzed")]);
        add_rule!(special, sv!(b"breathalyser"), [sv!(b"breathalyzer")]);
        add_rule!(special, sv!(b"breathalysers"), [sv!(b"breathalyzers")]);
        add_rule!(special, sv!(b"breathalyses"), [sv!(b"breathalyzes")]);
        add_rule!(special, sv!(b"breathalysing"), [sv!(b"breathalyzing")]);
        add_rule!(special, sv!(b"brutalise"), [sv!(b"brutalize")]);
        add_rule!(special, sv!(b"brutalised"), [sv!(b"brutalized")]);
        add_rule!(special, sv!(b"brutalises"), [sv!(b"brutalizes")]);
        add_rule!(special, sv!(b"brutalising"), [sv!(b"brutalizing")]);
        add_rule!(special, sv!(b"buses"), [sv!(b"busses")]);
        add_rule!(special, sv!(b"busing"), [sv!(b"bussing")]);
        add_rule!(special, sv!(b"caesarean"), [sv!(b"cesarean")]);
        add_rule!(special, sv!(b"caesareans"), [sv!(b"cesareans")]);
        add_rule!(special, sv!(b"calibre"), [sv!(b"caliber")]);
        add_rule!(special, sv!(b"calibres"), [sv!(b"calibers")]);
        add_rule!(special, sv!(b"calliper"), [sv!(b"caliper")]);
        add_rule!(special, sv!(b"callipers"), [sv!(b"calipers")]);
        add_rule!(special, sv!(b"callisthenics"), [sv!(b"calisthenics")]);
        add_rule!(special, sv!(b"canalise"), [sv!(b"canalize")]);
        add_rule!(special, sv!(b"canalised"), [sv!(b"canalized")]);
        add_rule!(special, sv!(b"canalises"), [sv!(b"canalizes")]);
        add_rule!(special, sv!(b"canalising"), [sv!(b"canalizing")]);
        add_rule!(special, sv!(b"cancellation"), [sv!(b"cancelation")]);
        add_rule!(special, sv!(b"cancellations"), [sv!(b"cancelations")]);
        add_rule!(special, sv!(b"cancelled"), [sv!(b"canceled")]);
        add_rule!(special, sv!(b"cancelling"), [sv!(b"canceling")]);
        add_rule!(special, sv!(b"candour"), [sv!(b"candor")]);
        add_rule!(special, sv!(b"cannibalise"), [sv!(b"cannibalize")]);
        add_rule!(special, sv!(b"cannibalised"), [sv!(b"cannibalized")]);
        add_rule!(special, sv!(b"cannibalises"), [sv!(b"cannibalizes")]);
        add_rule!(special, sv!(b"cannibalising"), [sv!(b"cannibalizing")]);
        add_rule!(special, sv!(b"canonise"), [sv!(b"canonize")]);
        add_rule!(special, sv!(b"canonised"), [sv!(b"canonized")]);
        add_rule!(special, sv!(b"canonises"), [sv!(b"canonizes")]);
        add_rule!(special, sv!(b"canonising"), [sv!(b"canonizing")]);
        add_rule!(special, sv!(b"capitalise"), [sv!(b"capitalize")]);
        add_rule!(special, sv!(b"capitalised"), [sv!(b"capitalized")]);
        add_rule!(special, sv!(b"capitalises"), [sv!(b"capitalizes")]);
        add_rule!(special, sv!(b"capitalising"), [sv!(b"capitalizing")]);
        add_rule!(special, sv!(b"caramelise"), [sv!(b"caramelize")]);
        add_rule!(special, sv!(b"caramelised"), [sv!(b"caramelized")]);
        add_rule!(special, sv!(b"caramelises"), [sv!(b"caramelizes")]);
        add_rule!(special, sv!(b"caramelising"), [sv!(b"caramelizing")]);
        add_rule!(special, sv!(b"carbonise"), [sv!(b"carbonize")]);
        add_rule!(special, sv!(b"carbonised"), [sv!(b"carbonized")]);
        add_rule!(special, sv!(b"carbonises"), [sv!(b"carbonizes")]);
        add_rule!(special, sv!(b"carbonising"), [sv!(b"carbonizing")]);
        add_rule!(special, sv!(b"carolled"), [sv!(b"caroled")]);
        add_rule!(special, sv!(b"carolling"), [sv!(b"caroling")]);
        add_rule!(special, sv!(b"catalogue"), [sv!(b"catalog")]);
        add_rule!(special, sv!(b"catalogued"), [sv!(b"cataloged")]);
        add_rule!(special, sv!(b"catalogues"), [sv!(b"catalogs")]);
        add_rule!(special, sv!(b"cataloguing"), [sv!(b"cataloging")]);
        add_rule!(special, sv!(b"catalyse"), [sv!(b"catalyze")]);
        add_rule!(special, sv!(b"catalysed"), [sv!(b"catalyzed")]);
        add_rule!(special, sv!(b"catalyses"), [sv!(b"catalyzes")]);
        add_rule!(special, sv!(b"catalysing"), [sv!(b"catalyzing")]);
        add_rule!(special, sv!(b"categorise"), [sv!(b"categorize")]);
        add_rule!(special, sv!(b"categorised"), [sv!(b"categorized")]);
        add_rule!(special, sv!(b"categorises"), [sv!(b"categorizes")]);
        add_rule!(special, sv!(b"categorising"), [sv!(b"categorizing")]);
        add_rule!(special, sv!(b"cauterise"), [sv!(b"cauterize")]);
        add_rule!(special, sv!(b"cauterised"), [sv!(b"cauterized")]);
        add_rule!(special, sv!(b"cauterises"), [sv!(b"cauterizes")]);
        add_rule!(special, sv!(b"cauterising"), [sv!(b"cauterizing")]);
        add_rule!(special, sv!(b"cavilled"), [sv!(b"caviled")]);
        add_rule!(special, sv!(b"cavilling"), [sv!(b"caviling")]);
        add_rule!(special, sv!(b"centigramme"), [sv!(b"centigram")]);
        add_rule!(special, sv!(b"centigrammes"), [sv!(b"centigrams")]);
        add_rule!(special, sv!(b"centilitre"), [sv!(b"centiliter")]);
        add_rule!(special, sv!(b"centilitres"), [sv!(b"centiliters")]);
        add_rule!(special, sv!(b"centimetre"), [sv!(b"centimeter")]);
        add_rule!(special, sv!(b"centimetres"), [sv!(b"centimeters")]);
        add_rule!(special, sv!(b"centralise"), [sv!(b"centralize")]);
        add_rule!(special, sv!(b"centralised"), [sv!(b"centralized")]);
        add_rule!(special, sv!(b"centralises"), [sv!(b"centralizes")]);
        add_rule!(special, sv!(b"centralising"), [sv!(b"centralizing")]);
        add_rule!(special, sv!(b"centre"), [sv!(b"center")]);
        add_rule!(special, sv!(b"centred"), [sv!(b"centered")]);
        add_rule!(special, sv!(b"centrefold"), [sv!(b"centerfold")]);
        add_rule!(special, sv!(b"centrefolds"), [sv!(b"centerfolds")]);
        add_rule!(special, sv!(b"centrepiece"), [sv!(b"centerpiece")]);
        add_rule!(special, sv!(b"centrepieces"), [sv!(b"centerpieces")]);
        add_rule!(special, sv!(b"centres"), [sv!(b"centers")]);
        add_rule!(special, sv!(b"channelled"), [sv!(b"channeled")]);
        add_rule!(special, sv!(b"channelling"), [sv!(b"channeling")]);
        add_rule!(special, sv!(b"characterise"), [sv!(b"characterize")]);
        add_rule!(special, sv!(b"characterised"), [sv!(b"characterized")]);
        add_rule!(special, sv!(b"characterises"), [sv!(b"characterizes")]);
        add_rule!(special, sv!(b"characterising"), [sv!(b"characterizing")]);
        add_rule!(special, sv!(b"cheque"), [sv!(b"check")]);
        add_rule!(special, sv!(b"chequebook"), [sv!(b"checkbook")]);
        add_rule!(special, sv!(b"chequebooks"), [sv!(b"checkbooks")]);
        add_rule!(special, sv!(b"chequered"), [sv!(b"checkered")]);
        add_rule!(special, sv!(b"cheques"), [sv!(b"checks")]);
        add_rule!(special, sv!(b"chilli"), [sv!(b"chili")]);
        add_rule!(special, sv!(b"chimaera"), [sv!(b"chimera")]);
        add_rule!(special, sv!(b"chimaeras"), [sv!(b"chimeras")]);
        add_rule!(special, sv!(b"chiselled"), [sv!(b"chiseled")]);
        add_rule!(special, sv!(b"chiselling"), [sv!(b"chiseling")]);
        add_rule!(special, sv!(b"circularise"), [sv!(b"circularize")]);
        add_rule!(special, sv!(b"circularised"), [sv!(b"circularized")]);
        add_rule!(special, sv!(b"circularises"), [sv!(b"circularizes")]);
        add_rule!(special, sv!(b"circularising"), [sv!(b"circularizing")]);
        add_rule!(special, sv!(b"civilise"), [sv!(b"civilize")]);
        add_rule!(special, sv!(b"civilised"), [sv!(b"civilized")]);
        add_rule!(special, sv!(b"civilises"), [sv!(b"civilizes")]);
        add_rule!(special, sv!(b"civilising"), [sv!(b"civilizing")]);
        add_rule!(special, sv!(b"clamour"), [sv!(b"clamor")]);
        add_rule!(special, sv!(b"clamoured"), [sv!(b"clamored")]);
        add_rule!(special, sv!(b"clamouring"), [sv!(b"clamoring")]);
        add_rule!(special, sv!(b"clamours"), [sv!(b"clamors")]);
        add_rule!(special, sv!(b"clangour"), [sv!(b"clangor")]);
        add_rule!(special, sv!(b"clarinettist"), [sv!(b"clarinetist")]);
        add_rule!(special, sv!(b"clarinettists"), [sv!(b"clarinetists")]);
        add_rule!(special, sv!(b"collectivise"), [sv!(b"collectivize")]);
        add_rule!(special, sv!(b"collectivised"), [sv!(b"collectivized")]);
        add_rule!(special, sv!(b"collectivises"), [sv!(b"collectivizes")]);
        add_rule!(special, sv!(b"collectivising"), [sv!(b"collectivizing")]);
        add_rule!(special, sv!(b"colonisation"), [sv!(b"colonization")]);
        add_rule!(special, sv!(b"colonise"), [sv!(b"colonize")]);
        add_rule!(special, sv!(b"colonised"), [sv!(b"colonized")]);
        add_rule!(special, sv!(b"coloniser"), [sv!(b"colonizer")]);
        add_rule!(special, sv!(b"colonisers"), [sv!(b"colonizers")]);
        add_rule!(special, sv!(b"colonises"), [sv!(b"colonizes")]);
        add_rule!(special, sv!(b"colonising"), [sv!(b"colonizing")]);
        add_rule!(special, sv!(b"colour"), [sv!(b"color")]);
        add_rule!(special, sv!(b"colourant"), [sv!(b"colorant")]);
        add_rule!(special, sv!(b"colourants"), [sv!(b"colorants")]);
        add_rule!(special, sv!(b"coloured"), [sv!(b"colored")]);
        add_rule!(special, sv!(b"coloureds"), [sv!(b"coloreds")]);
        add_rule!(special, sv!(b"colourful"), [sv!(b"colorful")]);
        add_rule!(special, sv!(b"colourfully"), [sv!(b"colorfully")]);
        add_rule!(special, sv!(b"colouring"), [sv!(b"coloring")]);
        add_rule!(special, sv!(b"colourize"), [sv!(b"colorize")]);
        add_rule!(special, sv!(b"colourized"), [sv!(b"colorized")]);
        add_rule!(special, sv!(b"colourizes"), [sv!(b"colorizes")]);
        add_rule!(special, sv!(b"colourizing"), [sv!(b"colorizing")]);
        add_rule!(special, sv!(b"colourless"), [sv!(b"colorless")]);
        add_rule!(special, sv!(b"colours"), [sv!(b"colors")]);
        add_rule!(special, sv!(b"commercialise"), [sv!(b"commercialize")]);
        add_rule!(special, sv!(b"commercialised"), [sv!(b"commercialized")]);
        add_rule!(special, sv!(b"commercialises"), [sv!(b"commercializes")]);
        add_rule!(special, sv!(b"commercialising"), [sv!(b"commercializing")]);
        add_rule!(
            special,
            sv!(b"compartmentalise"),
            [sv!(b"compartmentalize")]
        );
        add_rule!(
            special,
            sv!(b"compartmentalised"),
            [sv!(b"compartmentalized")]
        );
        add_rule!(
            special,
            sv!(b"compartmentalises"),
            [sv!(b"compartmentalizes")]
        );
        add_rule!(
            special,
            sv!(b"compartmentalising"),
            [sv!(b"compartmentalizing")]
        );
        add_rule!(special, sv!(b"computerise"), [sv!(b"computerize")]);
        add_rule!(special, sv!(b"computerised"), [sv!(b"computerized")]);
        add_rule!(special, sv!(b"computerises"), [sv!(b"computerizes")]);
        add_rule!(special, sv!(b"computerising"), [sv!(b"computerizing")]);
        add_rule!(special, sv!(b"conceptualise"), [sv!(b"conceptualize")]);
        add_rule!(special, sv!(b"conceptualised"), [sv!(b"conceptualized")]);
        add_rule!(special, sv!(b"conceptualises"), [sv!(b"conceptualizes")]);
        add_rule!(special, sv!(b"conceptualising"), [sv!(b"conceptualizing")]);
        add_rule!(special, sv!(b"connexion"), [sv!(b"connection")]);
        add_rule!(special, sv!(b"connexions"), [sv!(b"connections")]);
        add_rule!(special, sv!(b"contextualise"), [sv!(b"contextualize")]);
        add_rule!(special, sv!(b"contextualised"), [sv!(b"contextualized")]);
        add_rule!(special, sv!(b"contextualises"), [sv!(b"contextualizes")]);
        add_rule!(special, sv!(b"contextualising"), [sv!(b"contextualizing")]);
        add_rule!(special, sv!(b"cosier"), [sv!(b"cozier")]);
        add_rule!(special, sv!(b"cosies"), [sv!(b"cozies")]);
        add_rule!(special, sv!(b"cosiest"), [sv!(b"coziest")]);
        add_rule!(special, sv!(b"cosily"), [sv!(b"cozily")]);
        add_rule!(special, sv!(b"cosiness"), [sv!(b"coziness")]);
        add_rule!(special, sv!(b"cosy"), [sv!(b"cozy")]);
        add_rule!(special, sv!(b"councillor"), [sv!(b"councilor")]);
        add_rule!(special, sv!(b"councillors"), [sv!(b"councilors")]);
        add_rule!(special, sv!(b"counselled"), [sv!(b"counseled")]);
        add_rule!(special, sv!(b"counselling"), [sv!(b"counseling")]);
        add_rule!(special, sv!(b"counsellor"), [sv!(b"counselor")]);
        add_rule!(special, sv!(b"counsellors"), [sv!(b"counselors")]);
        add_rule!(special, sv!(b"crenellated"), [sv!(b"crenelated")]);
        add_rule!(special, sv!(b"criminalise"), [sv!(b"criminalize")]);
        add_rule!(special, sv!(b"criminalised"), [sv!(b"criminalized")]);
        add_rule!(special, sv!(b"criminalises"), [sv!(b"criminalizes")]);
        add_rule!(special, sv!(b"criminalising"), [sv!(b"criminalizing")]);
        add_rule!(special, sv!(b"criticise"), [sv!(b"criticize")]);
        add_rule!(special, sv!(b"criticised"), [sv!(b"criticized")]);
        add_rule!(special, sv!(b"criticises"), [sv!(b"criticizes")]);
        add_rule!(special, sv!(b"criticising"), [sv!(b"criticizing")]);
        add_rule!(special, sv!(b"crueller"), [sv!(b"crueler")]);
        add_rule!(special, sv!(b"cruellest"), [sv!(b"cruelest")]);
        add_rule!(special, sv!(b"crystallisation"), [sv!(b"crystallization")]);
        add_rule!(special, sv!(b"crystallise"), [sv!(b"crystallize")]);
        add_rule!(special, sv!(b"crystallised"), [sv!(b"crystallized")]);
        add_rule!(special, sv!(b"crystallises"), [sv!(b"crystallizes")]);
        add_rule!(special, sv!(b"crystallising"), [sv!(b"crystallizing")]);
        add_rule!(special, sv!(b"cudgelled"), [sv!(b"cudgeled")]);
        add_rule!(special, sv!(b"cudgelling"), [sv!(b"cudgeling")]);
        add_rule!(special, sv!(b"customise"), [sv!(b"customize")]);
        add_rule!(special, sv!(b"customised"), [sv!(b"customized")]);
        add_rule!(special, sv!(b"customises"), [sv!(b"customizes")]);
        add_rule!(special, sv!(b"customising"), [sv!(b"customizing")]);
        add_rule!(special, sv!(b"cypher"), [sv!(b"cipher")]);
        add_rule!(special, sv!(b"cyphers"), [sv!(b"ciphers")]);
        add_rule!(
            special,
            sv!(b"decentralisation"),
            [sv!(b"decentralization")]
        );
        add_rule!(special, sv!(b"decentralise"), [sv!(b"decentralize")]);
        add_rule!(special, sv!(b"decentralised"), [sv!(b"decentralized")]);
        add_rule!(special, sv!(b"decentralises"), [sv!(b"decentralizes")]);
        add_rule!(special, sv!(b"decentralising"), [sv!(b"decentralizing")]);
        add_rule!(
            special,
            sv!(b"decriminalisation"),
            [sv!(b"decriminalization")]
        );
        add_rule!(special, sv!(b"decriminalise"), [sv!(b"decriminalize")]);
        add_rule!(special, sv!(b"decriminalised"), [sv!(b"decriminalized")]);
        add_rule!(special, sv!(b"decriminalises"), [sv!(b"decriminalizes")]);
        add_rule!(special, sv!(b"decriminalising"), [sv!(b"decriminalizing")]);
        add_rule!(special, sv!(b"defence"), [sv!(b"defense")]);
        add_rule!(special, sv!(b"defenceless"), [sv!(b"defenseless")]);
        add_rule!(special, sv!(b"defences"), [sv!(b"defenses")]);
        add_rule!(special, sv!(b"dehumanisation"), [sv!(b"dehumanization")]);
        add_rule!(special, sv!(b"dehumanise"), [sv!(b"dehumanize")]);
        add_rule!(special, sv!(b"dehumanised"), [sv!(b"dehumanized")]);
        add_rule!(special, sv!(b"dehumanises"), [sv!(b"dehumanizes")]);
        add_rule!(special, sv!(b"dehumanising"), [sv!(b"dehumanizing")]);
        add_rule!(special, sv!(b"demeanour"), [sv!(b"demeanor")]);
        add_rule!(
            special,
            sv!(b"demilitarisation"),
            [sv!(b"demilitarization")]
        );
        add_rule!(special, sv!(b"demilitarise"), [sv!(b"demilitarize")]);
        add_rule!(special, sv!(b"demilitarised"), [sv!(b"demilitarized")]);
        add_rule!(special, sv!(b"demilitarises"), [sv!(b"demilitarizes")]);
        add_rule!(special, sv!(b"demilitarising"), [sv!(b"demilitarizing")]);
        add_rule!(special, sv!(b"demobilisation"), [sv!(b"demobilization")]);
        add_rule!(special, sv!(b"demobilise"), [sv!(b"demobilize")]);
        add_rule!(special, sv!(b"demobilised"), [sv!(b"demobilized")]);
        add_rule!(special, sv!(b"demobilises"), [sv!(b"demobilizes")]);
        add_rule!(special, sv!(b"demobilising"), [sv!(b"demobilizing")]);
        add_rule!(special, sv!(b"democratisation"), [sv!(b"democratization")]);
        add_rule!(special, sv!(b"democratise"), [sv!(b"democratize")]);
        add_rule!(special, sv!(b"democratised"), [sv!(b"democratized")]);
        add_rule!(special, sv!(b"democratises"), [sv!(b"democratizes")]);
        add_rule!(special, sv!(b"democratising"), [sv!(b"democratizing")]);
        add_rule!(special, sv!(b"demonise"), [sv!(b"demonize")]);
        add_rule!(special, sv!(b"demonised"), [sv!(b"demonized")]);
        add_rule!(special, sv!(b"demonises"), [sv!(b"demonizes")]);
        add_rule!(special, sv!(b"demonising"), [sv!(b"demonizing")]);
        add_rule!(special, sv!(b"demoralisation"), [sv!(b"demoralization")]);
        add_rule!(special, sv!(b"demoralise"), [sv!(b"demoralize")]);
        add_rule!(special, sv!(b"demoralised"), [sv!(b"demoralized")]);
        add_rule!(special, sv!(b"demoralises"), [sv!(b"demoralizes")]);
        add_rule!(special, sv!(b"demoralising"), [sv!(b"demoralizing")]);
        add_rule!(
            special,
            sv!(b"denationalisation"),
            [sv!(b"denationalization")]
        );
        add_rule!(special, sv!(b"denationalise"), [sv!(b"denationalize")]);
        add_rule!(special, sv!(b"denationalised"), [sv!(b"denationalized")]);
        add_rule!(special, sv!(b"denationalises"), [sv!(b"denationalizes")]);
        add_rule!(special, sv!(b"denationalising"), [sv!(b"denationalizing")]);
        add_rule!(special, sv!(b"deodorise"), [sv!(b"deodorize")]);
        add_rule!(special, sv!(b"deodorised"), [sv!(b"deodorized")]);
        add_rule!(special, sv!(b"deodorises"), [sv!(b"deodorizes")]);
        add_rule!(special, sv!(b"deodorising"), [sv!(b"deodorizing")]);
        add_rule!(special, sv!(b"depersonalise"), [sv!(b"depersonalize")]);
        add_rule!(special, sv!(b"depersonalised"), [sv!(b"depersonalized")]);
        add_rule!(special, sv!(b"depersonalises"), [sv!(b"depersonalizes")]);
        add_rule!(special, sv!(b"depersonalising"), [sv!(b"depersonalizing")]);
        add_rule!(special, sv!(b"deputise"), [sv!(b"deputize")]);
        add_rule!(special, sv!(b"deputised"), [sv!(b"deputized")]);
        add_rule!(special, sv!(b"deputises"), [sv!(b"deputizes")]);
        add_rule!(special, sv!(b"deputising"), [sv!(b"deputizing")]);
        add_rule!(special, sv!(b"desensitisation"), [sv!(b"desensitization")]);
        add_rule!(special, sv!(b"desensitise"), [sv!(b"desensitize")]);
        add_rule!(special, sv!(b"desensitised"), [sv!(b"desensitized")]);
        add_rule!(special, sv!(b"desensitises"), [sv!(b"desensitizes")]);
        add_rule!(special, sv!(b"desensitising"), [sv!(b"desensitizing")]);
        add_rule!(special, sv!(b"destabilisation"), [sv!(b"destabilization")]);
        add_rule!(special, sv!(b"destabilise"), [sv!(b"destabilize")]);
        add_rule!(special, sv!(b"destabilised"), [sv!(b"destabilized")]);
        add_rule!(special, sv!(b"destabilises"), [sv!(b"destabilizes")]);
        add_rule!(special, sv!(b"destabilising"), [sv!(b"destabilizing")]);
        add_rule!(special, sv!(b"dialled"), [sv!(b"dialed")]);
        add_rule!(special, sv!(b"dialling"), [sv!(b"dialing")]);
        add_rule!(special, sv!(b"dialogue"), [sv!(b"dialog")]);
        add_rule!(special, sv!(b"dialogues"), [sv!(b"dialogs")]);
        add_rule!(special, sv!(b"diarrhoea"), [sv!(b"diarrhea")]);
        add_rule!(special, sv!(b"digitise"), [sv!(b"digitize")]);
        add_rule!(special, sv!(b"digitised"), [sv!(b"digitized")]);
        add_rule!(special, sv!(b"digitises"), [sv!(b"digitizes")]);
        add_rule!(special, sv!(b"digitising"), [sv!(b"digitizing")]);
        add_rule!(special, sv!(b"disc"), [sv!(b"disk")]);
        add_rule!(special, sv!(b"discolour"), [sv!(b"discolor")]);
        add_rule!(special, sv!(b"discoloured"), [sv!(b"discolored")]);
        add_rule!(special, sv!(b"discolouring"), [sv!(b"discoloring")]);
        add_rule!(special, sv!(b"discolours"), [sv!(b"discolors")]);
        add_rule!(special, sv!(b"discs"), [sv!(b"disks")]);
        add_rule!(special, sv!(b"disembowelled"), [sv!(b"disemboweled")]);
        add_rule!(special, sv!(b"disembowelling"), [sv!(b"disemboweling")]);
        add_rule!(special, sv!(b"disfavour"), [sv!(b"disfavor")]);
        add_rule!(special, sv!(b"dishevelled"), [sv!(b"disheveled")]);
        add_rule!(special, sv!(b"dishonour"), [sv!(b"dishonor")]);
        add_rule!(special, sv!(b"dishonourable"), [sv!(b"dishonorable")]);
        add_rule!(special, sv!(b"dishonourably"), [sv!(b"dishonorably")]);
        add_rule!(special, sv!(b"dishonoured"), [sv!(b"dishonored")]);
        add_rule!(special, sv!(b"dishonouring"), [sv!(b"dishonoring")]);
        add_rule!(special, sv!(b"dishonours"), [sv!(b"dishonors")]);
        add_rule!(special, sv!(b"disorganisation"), [sv!(b"disorganization")]);
        add_rule!(special, sv!(b"disorganised"), [sv!(b"disorganized")]);
        add_rule!(special, sv!(b"distil"), [sv!(b"distill")]);
        add_rule!(special, sv!(b"distils"), [sv!(b"distills")]);
        add_rule!(special, sv!(b"doin"), [sv!(b"doing")]);
        add_rule!(special, sv!(b"doin'"), [sv!(b"doing")]);
        add_rule!(special, sv!(b"dramatisation"), [sv!(b"dramatization")]);
        add_rule!(special, sv!(b"dramatisations"), [sv!(b"dramatizations")]);
        add_rule!(special, sv!(b"dramatise"), [sv!(b"dramatize")]);
        add_rule!(special, sv!(b"dramatised"), [sv!(b"dramatized")]);
        add_rule!(special, sv!(b"dramatises"), [sv!(b"dramatizes")]);
        add_rule!(special, sv!(b"dramatising"), [sv!(b"dramatizing")]);
        add_rule!(special, sv!(b"draught"), [sv!(b"draft")]);
        add_rule!(special, sv!(b"draughtboard"), [sv!(b"draftboard")]);
        add_rule!(special, sv!(b"draughtboards"), [sv!(b"draftboards")]);
        add_rule!(special, sv!(b"draughtier"), [sv!(b"draftier")]);
        add_rule!(special, sv!(b"draughtiest"), [sv!(b"draftiest")]);
        add_rule!(special, sv!(b"draughts"), [sv!(b"drafts")]);
        add_rule!(special, sv!(b"draughtsman"), [sv!(b"draftsman")]);
        add_rule!(special, sv!(b"draughtsmanship"), [sv!(b"draftsmanship")]);
        add_rule!(special, sv!(b"draughtsmen"), [sv!(b"draftsmen")]);
        add_rule!(special, sv!(b"draughtswoman"), [sv!(b"draftswoman")]);
        add_rule!(special, sv!(b"draughtswomen"), [sv!(b"draftswomen")]);
        add_rule!(special, sv!(b"draughty"), [sv!(b"drafty")]);
        add_rule!(special, sv!(b"drivelled"), [sv!(b"driveled")]);
        add_rule!(special, sv!(b"drivelling"), [sv!(b"driveling")]);
        add_rule!(special, sv!(b"duelled"), [sv!(b"dueled")]);
        add_rule!(special, sv!(b"duelling"), [sv!(b"dueling")]);
        add_rule!(special, sv!(b"economise"), [sv!(b"economize")]);
        add_rule!(special, sv!(b"economised"), [sv!(b"economized")]);
        add_rule!(special, sv!(b"economises"), [sv!(b"economizes")]);
        add_rule!(special, sv!(b"economising"), [sv!(b"economizing")]);
        add_rule!(special, sv!(b"edoema"), [sv!(b"edema ")]);
        add_rule!(special, sv!(b"editorialise"), [sv!(b"editorialize")]);
        add_rule!(special, sv!(b"editorialised"), [sv!(b"editorialized")]);
        add_rule!(special, sv!(b"editorialises"), [sv!(b"editorializes")]);
        add_rule!(special, sv!(b"editorialising"), [sv!(b"editorializing")]);
        add_rule!(special, sv!(b"empathise"), [sv!(b"empathize")]);
        add_rule!(special, sv!(b"empathised"), [sv!(b"empathized")]);
        add_rule!(special, sv!(b"empathises"), [sv!(b"empathizes")]);
        add_rule!(special, sv!(b"empathising"), [sv!(b"empathizing")]);
        add_rule!(special, sv!(b"emphasise"), [sv!(b"emphasize")]);
        add_rule!(special, sv!(b"emphasised"), [sv!(b"emphasized")]);
        add_rule!(special, sv!(b"emphasises"), [sv!(b"emphasizes")]);
        add_rule!(special, sv!(b"emphasising"), [sv!(b"emphasizing")]);
        add_rule!(special, sv!(b"enamelled"), [sv!(b"enameled")]);
        add_rule!(special, sv!(b"enamelling"), [sv!(b"enameling")]);
        add_rule!(special, sv!(b"enamoured"), [sv!(b"enamored")]);
        add_rule!(special, sv!(b"encyclopaedia"), [sv!(b"encyclopedia")]);
        add_rule!(special, sv!(b"encyclopaedias"), [sv!(b"encyclopedias")]);
        add_rule!(special, sv!(b"encyclopaedic"), [sv!(b"encyclopedic")]);
        add_rule!(special, sv!(b"endeavour"), [sv!(b"endeavor")]);
        add_rule!(special, sv!(b"endeavoured"), [sv!(b"endeavored")]);
        add_rule!(special, sv!(b"endeavouring"), [sv!(b"endeavoring")]);
        add_rule!(special, sv!(b"endeavours"), [sv!(b"endeavors")]);
        add_rule!(special, sv!(b"energise"), [sv!(b"energize")]);
        add_rule!(special, sv!(b"energised"), [sv!(b"energized")]);
        add_rule!(special, sv!(b"energises"), [sv!(b"energizes")]);
        add_rule!(special, sv!(b"energising"), [sv!(b"energizing")]);
        add_rule!(special, sv!(b"enrol"), [sv!(b"enroll")]);
        add_rule!(special, sv!(b"enrols"), [sv!(b"enrolls")]);
        add_rule!(special, sv!(b"enthral"), [sv!(b"enthrall")]);
        add_rule!(special, sv!(b"enthrals"), [sv!(b"enthralls")]);
        add_rule!(special, sv!(b"epaulette"), [sv!(b"epaulet")]);
        add_rule!(special, sv!(b"epaulettes"), [sv!(b"epaulets")]);
        add_rule!(special, sv!(b"epicentre"), [sv!(b"epicenter")]);
        add_rule!(special, sv!(b"epicentres"), [sv!(b"epicenters")]);
        add_rule!(special, sv!(b"epilogue"), [sv!(b"epilog")]);
        add_rule!(special, sv!(b"epilogues"), [sv!(b"epilogs")]);
        add_rule!(special, sv!(b"epitomise"), [sv!(b"epitomize")]);
        add_rule!(special, sv!(b"epitomised"), [sv!(b"epitomized")]);
        add_rule!(special, sv!(b"epitomises"), [sv!(b"epitomizes")]);
        add_rule!(special, sv!(b"epitomising"), [sv!(b"epitomizing")]);
        add_rule!(special, sv!(b"equalisation"), [sv!(b"equalization")]);
        add_rule!(special, sv!(b"equalise"), [sv!(b"equalize")]);
        add_rule!(special, sv!(b"equalised"), [sv!(b"equalized")]);
        add_rule!(special, sv!(b"equaliser"), [sv!(b"equalizer")]);
        add_rule!(special, sv!(b"equalisers"), [sv!(b"equalizers")]);
        add_rule!(special, sv!(b"equalises"), [sv!(b"equalizes")]);
        add_rule!(special, sv!(b"equalising"), [sv!(b"equalizing")]);
        add_rule!(special, sv!(b"eulogise"), [sv!(b"eulogize")]);
        add_rule!(special, sv!(b"eulogised"), [sv!(b"eulogized")]);
        add_rule!(special, sv!(b"eulogises"), [sv!(b"eulogizes")]);
        add_rule!(special, sv!(b"eulogising"), [sv!(b"eulogizing")]);
        add_rule!(special, sv!(b"evangelise"), [sv!(b"evangelize")]);
        add_rule!(special, sv!(b"evangelised"), [sv!(b"evangelized")]);
        add_rule!(special, sv!(b"evangelises"), [sv!(b"evangelizes")]);
        add_rule!(special, sv!(b"evangelising"), [sv!(b"evangelizing")]);
        add_rule!(special, sv!(b"exorcise"), [sv!(b"exorcize")]);
        add_rule!(special, sv!(b"exorcised"), [sv!(b"exorcized")]);
        add_rule!(special, sv!(b"exorcises"), [sv!(b"exorcizes")]);
        add_rule!(special, sv!(b"exorcising"), [sv!(b"exorcizing")]);
        add_rule!(special, sv!(b"extemporisation"), [sv!(b"extemporization")]);
        add_rule!(special, sv!(b"extemporise"), [sv!(b"extemporize")]);
        add_rule!(special, sv!(b"extemporised"), [sv!(b"extemporized")]);
        add_rule!(special, sv!(b"extemporises"), [sv!(b"extemporizes")]);
        add_rule!(special, sv!(b"extemporising"), [sv!(b"extemporizing")]);
        add_rule!(special, sv!(b"externalisation"), [sv!(b"externalization")]);
        add_rule!(
            special,
            sv!(b"externalisations"),
            [sv!(b"externalizations")]
        );
        add_rule!(special, sv!(b"externalise"), [sv!(b"externalize")]);
        add_rule!(special, sv!(b"externalised"), [sv!(b"externalized")]);
        add_rule!(special, sv!(b"externalises"), [sv!(b"externalizes")]);
        add_rule!(special, sv!(b"externalising"), [sv!(b"externalizing")]);
        add_rule!(special, sv!(b"factorise"), [sv!(b"factorize")]);
        add_rule!(special, sv!(b"factorised"), [sv!(b"factorized")]);
        add_rule!(special, sv!(b"factorises"), [sv!(b"factorizes")]);
        add_rule!(special, sv!(b"factorising"), [sv!(b"factorizing")]);
        add_rule!(special, sv!(b"faecal"), [sv!(b"fecal")]);
        add_rule!(special, sv!(b"faeces"), [sv!(b"feces")]);
        add_rule!(special, sv!(b"familiarisation"), [sv!(b"familiarization")]);
        add_rule!(special, sv!(b"familiarise"), [sv!(b"familiarize")]);
        add_rule!(special, sv!(b"familiarised"), [sv!(b"familiarized")]);
        add_rule!(special, sv!(b"familiarises"), [sv!(b"familiarizes")]);
        add_rule!(special, sv!(b"familiarising"), [sv!(b"familiarizing")]);
        add_rule!(special, sv!(b"fantasise"), [sv!(b"fantasize")]);
        add_rule!(special, sv!(b"fantasised"), [sv!(b"fantasized")]);
        add_rule!(special, sv!(b"fantasises"), [sv!(b"fantasizes")]);
        add_rule!(special, sv!(b"fantasising"), [sv!(b"fantasizing")]);
        add_rule!(special, sv!(b"favour"), [sv!(b"favor")]);
        add_rule!(special, sv!(b"favourable"), [sv!(b"favorable")]);
        add_rule!(special, sv!(b"favourably"), [sv!(b"favorably")]);
        add_rule!(special, sv!(b"favoured"), [sv!(b"favored")]);
        add_rule!(special, sv!(b"favouring"), [sv!(b"favoring")]);
        add_rule!(special, sv!(b"favourite"), [sv!(b"favorite")]);
        add_rule!(special, sv!(b"favourites"), [sv!(b"favorites")]);
        add_rule!(special, sv!(b"favouritism"), [sv!(b"favoritism")]);
        add_rule!(special, sv!(b"favours"), [sv!(b"favors")]);
        add_rule!(special, sv!(b"feminise"), [sv!(b"feminize")]);
        add_rule!(special, sv!(b"feminised"), [sv!(b"feminized")]);
        add_rule!(special, sv!(b"feminises"), [sv!(b"feminizes")]);
        add_rule!(special, sv!(b"feminising"), [sv!(b"feminizing")]);
        add_rule!(special, sv!(b"fertilisation"), [sv!(b"fertilization")]);
        add_rule!(special, sv!(b"fertilise"), [sv!(b"fertilize")]);
        add_rule!(special, sv!(b"fertilised"), [sv!(b"fertilized")]);
        add_rule!(special, sv!(b"fertiliser"), [sv!(b"fertilizer")]);
        add_rule!(special, sv!(b"fertilisers"), [sv!(b"fertilizers")]);
        add_rule!(special, sv!(b"fertilises"), [sv!(b"fertilizes")]);
        add_rule!(special, sv!(b"fertilising"), [sv!(b"fertilizing")]);
        add_rule!(special, sv!(b"fervour"), [sv!(b"fervor")]);
        add_rule!(special, sv!(b"fibre"), [sv!(b"fiber")]);
        add_rule!(special, sv!(b"fibreglass"), [sv!(b"fiberglass")]);
        add_rule!(special, sv!(b"fibres"), [sv!(b"fibers")]);
        add_rule!(
            special,
            sv!(b"fictionalisation"),
            [sv!(b"fictionalization")]
        );
        add_rule!(
            special,
            sv!(b"fictionalisations"),
            [sv!(b"fictionalizations")]
        );
        add_rule!(special, sv!(b"fictionalise"), [sv!(b"fictionalize")]);
        add_rule!(special, sv!(b"fictionalised"), [sv!(b"fictionalized")]);
        add_rule!(special, sv!(b"fictionalises"), [sv!(b"fictionalizes")]);
        add_rule!(special, sv!(b"fictionalising"), [sv!(b"fictionalizing")]);
        add_rule!(special, sv!(b"fillet"), [sv!(b"filet")]);
        add_rule!(special, sv!(b"filleted "), [sv!(b"fileted ")]);
        add_rule!(special, sv!(b"filleting"), [sv!(b"fileting")]);
        add_rule!(special, sv!(b"fillets "), [sv!(b"filets ")]);
        add_rule!(special, sv!(b"finalisation"), [sv!(b"finalization")]);
        add_rule!(special, sv!(b"finalise"), [sv!(b"finalize")]);
        add_rule!(special, sv!(b"finalised"), [sv!(b"finalized")]);
        add_rule!(special, sv!(b"finalises"), [sv!(b"finalizes")]);
        add_rule!(special, sv!(b"finalising"), [sv!(b"finalizing")]);
        add_rule!(special, sv!(b"flautist"), [sv!(b"flutist")]);
        add_rule!(special, sv!(b"flautists"), [sv!(b"flutists")]);
        add_rule!(special, sv!(b"flavour"), [sv!(b"flavor")]);
        add_rule!(special, sv!(b"flavoured"), [sv!(b"flavored")]);
        add_rule!(special, sv!(b"flavouring"), [sv!(b"flavoring")]);
        add_rule!(special, sv!(b"flavourings"), [sv!(b"flavorings")]);
        add_rule!(special, sv!(b"flavourless"), [sv!(b"flavorless")]);
        add_rule!(special, sv!(b"flavours"), [sv!(b"flavors")]);
        add_rule!(special, sv!(b"flavoursome"), [sv!(b"flavorsome")]);
        add_rule!(special, sv!(b"flyer / flier "), [sv!(b"flier / flyer ")]);
        add_rule!(special, sv!(b"foetal"), [sv!(b"fetal")]);
        add_rule!(special, sv!(b"foetid"), [sv!(b"fetid")]);
        add_rule!(special, sv!(b"foetus"), [sv!(b"fetus")]);
        add_rule!(special, sv!(b"foetuses"), [sv!(b"fetuses")]);
        add_rule!(special, sv!(b"formalisation"), [sv!(b"formalization")]);
        add_rule!(special, sv!(b"formalise"), [sv!(b"formalize")]);
        add_rule!(special, sv!(b"formalised"), [sv!(b"formalized")]);
        add_rule!(special, sv!(b"formalises"), [sv!(b"formalizes")]);
        add_rule!(special, sv!(b"formalising"), [sv!(b"formalizing")]);
        add_rule!(special, sv!(b"fossilisation"), [sv!(b"fossilization")]);
        add_rule!(special, sv!(b"fossilise"), [sv!(b"fossilize")]);
        add_rule!(special, sv!(b"fossilised"), [sv!(b"fossilized")]);
        add_rule!(special, sv!(b"fossilises"), [sv!(b"fossilizes")]);
        add_rule!(special, sv!(b"fossilising"), [sv!(b"fossilizing")]);
        add_rule!(special, sv!(b"fraternisation"), [sv!(b"fraternization")]);
        add_rule!(special, sv!(b"fraternise"), [sv!(b"fraternize")]);
        add_rule!(special, sv!(b"fraternised"), [sv!(b"fraternized")]);
        add_rule!(special, sv!(b"fraternises"), [sv!(b"fraternizes")]);
        add_rule!(special, sv!(b"fraternising"), [sv!(b"fraternizing")]);
        add_rule!(special, sv!(b"fulfil"), [sv!(b"fulfill")]);
        add_rule!(special, sv!(b"fulfilment"), [sv!(b"fulfillment")]);
        add_rule!(special, sv!(b"fulfils"), [sv!(b"fulfills")]);
        add_rule!(special, sv!(b"funnelled"), [sv!(b"funneled")]);
        add_rule!(special, sv!(b"funnelling"), [sv!(b"funneling")]);
        add_rule!(special, sv!(b"galvanise"), [sv!(b"galvanize")]);
        add_rule!(special, sv!(b"galvanised"), [sv!(b"galvanized")]);
        add_rule!(special, sv!(b"galvanises"), [sv!(b"galvanizes")]);
        add_rule!(special, sv!(b"galvanising"), [sv!(b"galvanizing")]);
        add_rule!(special, sv!(b"gambolled"), [sv!(b"gamboled")]);
        add_rule!(special, sv!(b"gambolling"), [sv!(b"gamboling")]);
        add_rule!(special, sv!(b"gaol"), [sv!(b"jail")]);
        add_rule!(special, sv!(b"gaolbird"), [sv!(b"jailbird")]);
        add_rule!(special, sv!(b"gaolbirds"), [sv!(b"jailbirds")]);
        add_rule!(special, sv!(b"gaolbreak"), [sv!(b"jailbreak")]);
        add_rule!(special, sv!(b"gaolbreaks"), [sv!(b"jailbreaks")]);
        add_rule!(special, sv!(b"gaoled"), [sv!(b"jailed")]);
        add_rule!(special, sv!(b"gaoler"), [sv!(b"jailer")]);
        add_rule!(special, sv!(b"gaolers"), [sv!(b"jailers")]);
        add_rule!(special, sv!(b"gaoling"), [sv!(b"jailing")]);
        add_rule!(special, sv!(b"gaols"), [sv!(b"jails")]);
        add_rule!(special, sv!(b"gases"), [sv!(b"gasses")]);
        add_rule!(special, sv!(b"gauge"), [sv!(b"gage")]);
        add_rule!(special, sv!(b"gauged"), [sv!(b"gaged")]);
        add_rule!(special, sv!(b"gauges"), [sv!(b"gages")]);
        add_rule!(special, sv!(b"gauging"), [sv!(b"gaging")]);
        add_rule!(special, sv!(b"generalisation"), [sv!(b"generalization")]);
        add_rule!(special, sv!(b"generalisations"), [sv!(b"generalizations")]);
        add_rule!(special, sv!(b"generalise"), [sv!(b"generalize")]);
        add_rule!(special, sv!(b"generalised"), [sv!(b"generalized")]);
        add_rule!(special, sv!(b"generalises"), [sv!(b"generalizes")]);
        add_rule!(special, sv!(b"generalising"), [sv!(b"generalizing")]);
        add_rule!(special, sv!(b"ghettoise"), [sv!(b"ghettoize")]);
        add_rule!(special, sv!(b"ghettoised"), [sv!(b"ghettoized")]);
        add_rule!(special, sv!(b"ghettoises"), [sv!(b"ghettoizes")]);
        add_rule!(special, sv!(b"ghettoising"), [sv!(b"ghettoizing")]);
        add_rule!(special, sv!(b"gipsies"), [sv!(b"gypsies")]);
        add_rule!(special, sv!(b"glamorise"), [sv!(b"glamorize")]);
        add_rule!(special, sv!(b"glamorised"), [sv!(b"glamorized")]);
        add_rule!(special, sv!(b"glamorises"), [sv!(b"glamorizes")]);
        add_rule!(special, sv!(b"glamorising"), [sv!(b"glamorizing")]);
        add_rule!(special, sv!(b"glamour"), [sv!(b"glamor")]);
        add_rule!(special, sv!(b"globalisation"), [sv!(b"globalization")]);
        add_rule!(special, sv!(b"globalise"), [sv!(b"globalize")]);
        add_rule!(special, sv!(b"globalised"), [sv!(b"globalized")]);
        add_rule!(special, sv!(b"globalises"), [sv!(b"globalizes")]);
        add_rule!(special, sv!(b"globalising"), [sv!(b"globalizing")]);
        add_rule!(special, sv!(b"glueing "), [sv!(b"gluing ")]);
        add_rule!(special, sv!(b"goin"), [sv!(b"going")]);
        add_rule!(special, sv!(b"goin'"), [sv!(b"going")]);
        add_rule!(special, sv!(b"goitre"), [sv!(b"goiter")]);
        add_rule!(special, sv!(b"goitres"), [sv!(b"goiters")]);
        add_rule!(special, sv!(b"gonorrhoea"), [sv!(b"gonorrhea")]);
        add_rule!(special, sv!(b"gramme"), [sv!(b"gram")]);
        add_rule!(special, sv!(b"grammes"), [sv!(b"grams")]);
        add_rule!(special, sv!(b"gravelled"), [sv!(b"graveled")]);
        add_rule!(special, sv!(b"grey"), [sv!(b"gray")]);
        add_rule!(special, sv!(b"greyed"), [sv!(b"grayed")]);
        add_rule!(special, sv!(b"greying"), [sv!(b"graying")]);
        add_rule!(special, sv!(b"greyish"), [sv!(b"grayish")]);
        add_rule!(special, sv!(b"greyness"), [sv!(b"grayness")]);
        add_rule!(special, sv!(b"greys"), [sv!(b"grays")]);
        add_rule!(special, sv!(b"grovelled"), [sv!(b"groveled")]);
        add_rule!(special, sv!(b"grovelling"), [sv!(b"groveling")]);
        add_rule!(special, sv!(b"groyne"), [sv!(b"groin")]);
        add_rule!(special, sv!(b"groynes "), [sv!(b"groins")]);
        add_rule!(special, sv!(b"gruelling"), [sv!(b"grueling")]);
        add_rule!(special, sv!(b"gruellingly"), [sv!(b"gruelingly")]);
        add_rule!(special, sv!(b"gryphon"), [sv!(b"griffin")]);
        add_rule!(special, sv!(b"gryphons"), [sv!(b"griffins")]);
        add_rule!(special, sv!(b"gynaecological"), [sv!(b"gynecological")]);
        add_rule!(special, sv!(b"gynaecologist"), [sv!(b"gynecologist")]);
        add_rule!(special, sv!(b"gynaecologists"), [sv!(b"gynecologists")]);
        add_rule!(special, sv!(b"gynaecology"), [sv!(b"gynecology")]);
        add_rule!(special, sv!(b"haematological"), [sv!(b"hematological")]);
        add_rule!(special, sv!(b"haematologist"), [sv!(b"hematologist")]);
        add_rule!(special, sv!(b"haematologists"), [sv!(b"hematologists")]);
        add_rule!(special, sv!(b"haematology"), [sv!(b"hematology")]);
        add_rule!(special, sv!(b"haemoglobin"), [sv!(b"hemoglobin")]);
        add_rule!(special, sv!(b"haemophilia"), [sv!(b"hemophilia")]);
        add_rule!(special, sv!(b"haemophiliac"), [sv!(b"hemophiliac")]);
        add_rule!(special, sv!(b"haemophiliacs"), [sv!(b"hemophiliacs")]);
        add_rule!(special, sv!(b"haemorrhage"), [sv!(b"hemorrhage")]);
        add_rule!(special, sv!(b"haemorrhaged"), [sv!(b"hemorrhaged")]);
        add_rule!(special, sv!(b"haemorrhages"), [sv!(b"hemorrhages")]);
        add_rule!(special, sv!(b"haemorrhaging"), [sv!(b"hemorrhaging")]);
        add_rule!(special, sv!(b"haemorrhoids"), [sv!(b"hemorrhoids")]);
        add_rule!(special, sv!(b"harbour"), [sv!(b"harbor")]);
        add_rule!(special, sv!(b"harboured"), [sv!(b"harbored")]);
        add_rule!(special, sv!(b"harbouring"), [sv!(b"harboring")]);
        add_rule!(special, sv!(b"harbours"), [sv!(b"harbors")]);
        add_rule!(special, sv!(b"harmonisation"), [sv!(b"harmonization")]);
        add_rule!(special, sv!(b"harmonise"), [sv!(b"harmonize")]);
        add_rule!(special, sv!(b"harmonised"), [sv!(b"harmonized")]);
        add_rule!(special, sv!(b"harmonises"), [sv!(b"harmonizes")]);
        add_rule!(special, sv!(b"harmonising"), [sv!(b"harmonizing")]);
        add_rule!(special, sv!(b"havin"), [sv!(b"having")]);
        add_rule!(special, sv!(b"havin'"), [sv!(b"having")]);
        add_rule!(special, sv!(b"homoeopath"), [sv!(b"homeopath")]);
        add_rule!(special, sv!(b"homoeopathic"), [sv!(b"homeopathic")]);
        add_rule!(special, sv!(b"homoeopaths"), [sv!(b"homeopaths")]);
        add_rule!(special, sv!(b"homoeopathy"), [sv!(b"homeopathy")]);
        add_rule!(special, sv!(b"homogenise"), [sv!(b"homogenize")]);
        add_rule!(special, sv!(b"homogenised"), [sv!(b"homogenized")]);
        add_rule!(special, sv!(b"homogenises"), [sv!(b"homogenizes")]);
        add_rule!(special, sv!(b"homogenising"), [sv!(b"homogenizing")]);
        add_rule!(special, sv!(b"honour"), [sv!(b"honor")]);
        add_rule!(special, sv!(b"honourable"), [sv!(b"honorable")]);
        add_rule!(special, sv!(b"honourably"), [sv!(b"honorably")]);
        add_rule!(special, sv!(b"honoured"), [sv!(b"honored")]);
        add_rule!(special, sv!(b"honouring"), [sv!(b"honoring")]);
        add_rule!(special, sv!(b"honours"), [sv!(b"honors")]);
        add_rule!(special, sv!(b"hospitalisation"), [sv!(b"hospitalization")]);
        add_rule!(special, sv!(b"hospitalise"), [sv!(b"hospitalize")]);
        add_rule!(special, sv!(b"hospitalised"), [sv!(b"hospitalized")]);
        add_rule!(special, sv!(b"hospitalises"), [sv!(b"hospitalizes")]);
        add_rule!(special, sv!(b"hospitalising"), [sv!(b"hospitalizing")]);
        add_rule!(special, sv!(b"humanise"), [sv!(b"humanize")]);
        add_rule!(special, sv!(b"humanised"), [sv!(b"humanized")]);
        add_rule!(special, sv!(b"humanises"), [sv!(b"humanizes")]);
        add_rule!(special, sv!(b"humanising"), [sv!(b"humanizing")]);
        add_rule!(special, sv!(b"humour"), [sv!(b"humor")]);
        add_rule!(special, sv!(b"humoured"), [sv!(b"humored")]);
        add_rule!(special, sv!(b"humouring"), [sv!(b"humoring")]);
        add_rule!(special, sv!(b"humourless"), [sv!(b"humorless")]);
        add_rule!(special, sv!(b"humours"), [sv!(b"humors")]);
        add_rule!(special, sv!(b"hybridise"), [sv!(b"hybridize")]);
        add_rule!(special, sv!(b"hybridised"), [sv!(b"hybridized")]);
        add_rule!(special, sv!(b"hybridises"), [sv!(b"hybridizes")]);
        add_rule!(special, sv!(b"hybridising"), [sv!(b"hybridizing")]);
        add_rule!(special, sv!(b"hypnotise"), [sv!(b"hypnotize")]);
        add_rule!(special, sv!(b"hypnotised"), [sv!(b"hypnotized")]);
        add_rule!(special, sv!(b"hypnotises"), [sv!(b"hypnotizes")]);
        add_rule!(special, sv!(b"hypnotising"), [sv!(b"hypnotizing")]);
        add_rule!(special, sv!(b"hypothesise"), [sv!(b"hypothesize")]);
        add_rule!(special, sv!(b"hypothesised"), [sv!(b"hypothesized")]);
        add_rule!(special, sv!(b"hypothesises"), [sv!(b"hypothesizes")]);
        add_rule!(special, sv!(b"hypothesising"), [sv!(b"hypothesizing")]);
        add_rule!(special, sv!(b"idealisation"), [sv!(b"idealization")]);
        add_rule!(special, sv!(b"idealise"), [sv!(b"idealize")]);
        add_rule!(special, sv!(b"idealised"), [sv!(b"idealized")]);
        add_rule!(special, sv!(b"idealises"), [sv!(b"idealizes")]);
        add_rule!(special, sv!(b"idealising"), [sv!(b"idealizing")]);
        add_rule!(special, sv!(b"idolise"), [sv!(b"idolize")]);
        add_rule!(special, sv!(b"idolised"), [sv!(b"idolized")]);
        add_rule!(special, sv!(b"idolises"), [sv!(b"idolizes")]);
        add_rule!(special, sv!(b"idolising"), [sv!(b"idolizing")]);
        add_rule!(special, sv!(b"immobilisation"), [sv!(b"immobilization")]);
        add_rule!(special, sv!(b"immobilise"), [sv!(b"immobilize")]);
        add_rule!(special, sv!(b"immobilised"), [sv!(b"immobilized")]);
        add_rule!(special, sv!(b"immobiliser"), [sv!(b"immobilizer")]);
        add_rule!(special, sv!(b"immobilisers"), [sv!(b"immobilizers")]);
        add_rule!(special, sv!(b"immobilises"), [sv!(b"immobilizes")]);
        add_rule!(special, sv!(b"immobilising"), [sv!(b"immobilizing")]);
        add_rule!(special, sv!(b"immortalise"), [sv!(b"immortalize")]);
        add_rule!(special, sv!(b"immortalised"), [sv!(b"immortalized")]);
        add_rule!(special, sv!(b"immortalises"), [sv!(b"immortalizes")]);
        add_rule!(special, sv!(b"immortalising"), [sv!(b"immortalizing")]);
        add_rule!(special, sv!(b"immunisation"), [sv!(b"immunization")]);
        add_rule!(special, sv!(b"immunise"), [sv!(b"immunize")]);
        add_rule!(special, sv!(b"immunised"), [sv!(b"immunized")]);
        add_rule!(special, sv!(b"immunises"), [sv!(b"immunizes")]);
        add_rule!(special, sv!(b"immunising"), [sv!(b"immunizing")]);
        add_rule!(special, sv!(b"impanelled"), [sv!(b"impaneled")]);
        add_rule!(special, sv!(b"impanelling"), [sv!(b"impaneling")]);
        add_rule!(special, sv!(b"imperilled"), [sv!(b"imperiled")]);
        add_rule!(special, sv!(b"imperilling"), [sv!(b"imperiling")]);
        add_rule!(special, sv!(b"individualise"), [sv!(b"individualize")]);
        add_rule!(special, sv!(b"individualised"), [sv!(b"individualized")]);
        add_rule!(special, sv!(b"individualises"), [sv!(b"individualizes")]);
        add_rule!(special, sv!(b"individualising"), [sv!(b"individualizing")]);
        add_rule!(special, sv!(b"industrialise"), [sv!(b"industrialize")]);
        add_rule!(special, sv!(b"industrialised"), [sv!(b"industrialized")]);
        add_rule!(special, sv!(b"industrialises"), [sv!(b"industrializes")]);
        add_rule!(special, sv!(b"industrialising"), [sv!(b"industrializing")]);
        add_rule!(special, sv!(b"inflexion"), [sv!(b"inflection")]);
        add_rule!(special, sv!(b"inflexions"), [sv!(b"inflections")]);
        add_rule!(special, sv!(b"initialise"), [sv!(b"initialize")]);
        add_rule!(special, sv!(b"initialised"), [sv!(b"initialized")]);
        add_rule!(special, sv!(b"initialises"), [sv!(b"initializes")]);
        add_rule!(special, sv!(b"initialising"), [sv!(b"initializing")]);
        add_rule!(special, sv!(b"initialled"), [sv!(b"initialed")]);
        add_rule!(special, sv!(b"initialling"), [sv!(b"initialing")]);
        add_rule!(special, sv!(b"instal"), [sv!(b"install")]);
        add_rule!(special, sv!(b"instalment"), [sv!(b"installment")]);
        add_rule!(special, sv!(b"instalments"), [sv!(b"installments")]);
        add_rule!(special, sv!(b"instals"), [sv!(b"installs")]);
        add_rule!(special, sv!(b"instil"), [sv!(b"instill")]);
        add_rule!(special, sv!(b"instils"), [sv!(b"instills")]);
        add_rule!(
            special,
            sv!(b"institutionalisation"),
            [sv!(b"institutionalization")]
        );
        add_rule!(
            special,
            sv!(b"institutionalise"),
            [sv!(b"institutionalize")]
        );
        add_rule!(
            special,
            sv!(b"institutionalised"),
            [sv!(b"institutionalized")]
        );
        add_rule!(
            special,
            sv!(b"institutionalises"),
            [sv!(b"institutionalizes")]
        );
        add_rule!(
            special,
            sv!(b"institutionalising"),
            [sv!(b"institutionalizing")]
        );
        add_rule!(special, sv!(b"intellectualise"), [sv!(b"intellectualize")]);
        add_rule!(
            special,
            sv!(b"intellectualised"),
            [sv!(b"intellectualized")]
        );
        add_rule!(
            special,
            sv!(b"intellectualises"),
            [sv!(b"intellectualizes")]
        );
        add_rule!(
            special,
            sv!(b"intellectualising"),
            [sv!(b"intellectualizing")]
        );
        add_rule!(special, sv!(b"internalisation"), [sv!(b"internalization")]);
        add_rule!(special, sv!(b"internalise"), [sv!(b"internalize")]);
        add_rule!(special, sv!(b"internalised"), [sv!(b"internalized")]);
        add_rule!(special, sv!(b"internalises"), [sv!(b"internalizes")]);
        add_rule!(special, sv!(b"internalising"), [sv!(b"internalizing")]);
        add_rule!(
            special,
            sv!(b"internationalisation"),
            [sv!(b"internationalization")]
        );
        add_rule!(
            special,
            sv!(b"internationalise"),
            [sv!(b"internationalize")]
        );
        add_rule!(
            special,
            sv!(b"internationalised"),
            [sv!(b"internationalized")]
        );
        add_rule!(
            special,
            sv!(b"internationalises"),
            [sv!(b"internationalizes")]
        );
        add_rule!(
            special,
            sv!(b"internationalising"),
            [sv!(b"internationalizing")]
        );
        add_rule!(special, sv!(b"ionisation"), [sv!(b"ionization")]);
        add_rule!(special, sv!(b"ionise"), [sv!(b"ionize")]);
        add_rule!(special, sv!(b"ionised"), [sv!(b"ionized")]);
        add_rule!(special, sv!(b"ioniser"), [sv!(b"ionizer")]);
        add_rule!(special, sv!(b"ionisers"), [sv!(b"ionizers")]);
        add_rule!(special, sv!(b"ionises"), [sv!(b"ionizes")]);
        add_rule!(special, sv!(b"ionising"), [sv!(b"ionizing")]);
        add_rule!(special, sv!(b"italicise"), [sv!(b"italicize")]);
        add_rule!(special, sv!(b"italicised"), [sv!(b"italicized")]);
        add_rule!(special, sv!(b"italicises"), [sv!(b"italicizes")]);
        add_rule!(special, sv!(b"italicising"), [sv!(b"italicizing")]);
        add_rule!(special, sv!(b"itemise"), [sv!(b"itemize")]);
        add_rule!(special, sv!(b"itemised"), [sv!(b"itemized")]);
        add_rule!(special, sv!(b"itemises"), [sv!(b"itemizes")]);
        add_rule!(special, sv!(b"itemising"), [sv!(b"itemizing")]);
        add_rule!(special, sv!(b"jeopardise"), [sv!(b"jeopardize")]);
        add_rule!(special, sv!(b"jeopardised"), [sv!(b"jeopardized")]);
        add_rule!(special, sv!(b"jeopardises"), [sv!(b"jeopardizes")]);
        add_rule!(special, sv!(b"jeopardising"), [sv!(b"jeopardizing")]);
        add_rule!(special, sv!(b"jewelled"), [sv!(b"jeweled")]);
        add_rule!(special, sv!(b"jeweller"), [sv!(b"jeweler")]);
        add_rule!(special, sv!(b"jewellers"), [sv!(b"jewelers")]);
        add_rule!(special, sv!(b"jewellery"), [sv!(b"jewelry")]);
        add_rule!(special, sv!(b"judgement "), [sv!(b"judgment")]);
        add_rule!(special, sv!(b"kilogramme"), [sv!(b"kilogram")]);
        add_rule!(special, sv!(b"kilogrammes"), [sv!(b"kilograms")]);
        add_rule!(special, sv!(b"kilometre"), [sv!(b"kilometer")]);
        add_rule!(special, sv!(b"kilometres"), [sv!(b"kilometers")]);
        add_rule!(special, sv!(b"labelled"), [sv!(b"labeled")]);
        add_rule!(special, sv!(b"labelling"), [sv!(b"labeling")]);
        add_rule!(special, sv!(b"labour"), [sv!(b"labor")]);
        add_rule!(special, sv!(b"laboured"), [sv!(b"labored")]);
        add_rule!(special, sv!(b"labourer"), [sv!(b"laborer")]);
        add_rule!(special, sv!(b"labourers"), [sv!(b"laborers")]);
        add_rule!(special, sv!(b"labouring"), [sv!(b"laboring")]);
        add_rule!(special, sv!(b"labours"), [sv!(b"labors")]);
        add_rule!(special, sv!(b"lacklustre"), [sv!(b"lackluster")]);
        add_rule!(special, sv!(b"legalisation"), [sv!(b"legalization")]);
        add_rule!(special, sv!(b"legalise"), [sv!(b"legalize")]);
        add_rule!(special, sv!(b"legalised"), [sv!(b"legalized")]);
        add_rule!(special, sv!(b"legalises"), [sv!(b"legalizes")]);
        add_rule!(special, sv!(b"legalising"), [sv!(b"legalizing")]);
        add_rule!(special, sv!(b"legitimise"), [sv!(b"legitimize")]);
        add_rule!(special, sv!(b"legitimised"), [sv!(b"legitimized")]);
        add_rule!(special, sv!(b"legitimises"), [sv!(b"legitimizes")]);
        add_rule!(special, sv!(b"legitimising"), [sv!(b"legitimizing")]);
        add_rule!(special, sv!(b"leukaemia"), [sv!(b"leukemia")]);
        add_rule!(special, sv!(b"levelled"), [sv!(b"leveled")]);
        add_rule!(special, sv!(b"leveller"), [sv!(b"leveler")]);
        add_rule!(special, sv!(b"levellers"), [sv!(b"levelers")]);
        add_rule!(special, sv!(b"levelling"), [sv!(b"leveling")]);
        add_rule!(special, sv!(b"libelled"), [sv!(b"libeled")]);
        add_rule!(special, sv!(b"libelling"), [sv!(b"libeling")]);
        add_rule!(special, sv!(b"libellous"), [sv!(b"libelous")]);
        add_rule!(special, sv!(b"liberalisation"), [sv!(b"liberalization")]);
        add_rule!(special, sv!(b"liberalise"), [sv!(b"liberalize")]);
        add_rule!(special, sv!(b"liberalised"), [sv!(b"liberalized")]);
        add_rule!(special, sv!(b"liberalises"), [sv!(b"liberalizes")]);
        add_rule!(special, sv!(b"liberalising"), [sv!(b"liberalizing")]);
        add_rule!(special, sv!(b"licence"), [sv!(b"license")]);
        add_rule!(special, sv!(b"licenced"), [sv!(b"licensed")]);
        add_rule!(special, sv!(b"licences"), [sv!(b"licenses")]);
        add_rule!(special, sv!(b"licencing"), [sv!(b"licensing")]);
        add_rule!(special, sv!(b"likeable"), [sv!(b"likable ")]);
        add_rule!(special, sv!(b"lionisation"), [sv!(b"lionization")]);
        add_rule!(special, sv!(b"lionise"), [sv!(b"lionize")]);
        add_rule!(special, sv!(b"lionised"), [sv!(b"lionized")]);
        add_rule!(special, sv!(b"lionises"), [sv!(b"lionizes")]);
        add_rule!(special, sv!(b"lionising"), [sv!(b"lionizing")]);
        add_rule!(special, sv!(b"liquidise"), [sv!(b"liquidize")]);
        add_rule!(special, sv!(b"liquidised"), [sv!(b"liquidized")]);
        add_rule!(special, sv!(b"liquidiser"), [sv!(b"liquidizer")]);
        add_rule!(special, sv!(b"liquidisers"), [sv!(b"liquidizers")]);
        add_rule!(special, sv!(b"liquidises"), [sv!(b"liquidizes")]);
        add_rule!(special, sv!(b"liquidising"), [sv!(b"liquidizing")]);
        add_rule!(special, sv!(b"litre"), [sv!(b"liter")]);
        add_rule!(special, sv!(b"litres"), [sv!(b"liters")]);
        add_rule!(special, sv!(b"localise"), [sv!(b"localize")]);
        add_rule!(special, sv!(b"localised"), [sv!(b"localized")]);
        add_rule!(special, sv!(b"localises"), [sv!(b"localizes")]);
        add_rule!(special, sv!(b"localising"), [sv!(b"localizing")]);
        add_rule!(special, sv!(b"lovin"), [sv!(b"loving")]);
        add_rule!(special, sv!(b"lovin'"), [sv!(b"loving")]);
        add_rule!(special, sv!(b"louvre"), [sv!(b"louver")]);
        add_rule!(special, sv!(b"louvred"), [sv!(b"louvered")]);
        add_rule!(special, sv!(b"louvres"), [sv!(b"louvers ")]);
        add_rule!(special, sv!(b"lustre"), [sv!(b"luster")]);
        add_rule!(special, sv!(b"magnetise"), [sv!(b"magnetize")]);
        add_rule!(special, sv!(b"magnetised"), [sv!(b"magnetized")]);
        add_rule!(special, sv!(b"magnetises"), [sv!(b"magnetizes")]);
        add_rule!(special, sv!(b"magnetising"), [sv!(b"magnetizing")]);
        add_rule!(special, sv!(b"manoeuvrability"), [sv!(b"maneuverability")]);
        add_rule!(special, sv!(b"manoeuvrable"), [sv!(b"maneuverable")]);
        add_rule!(special, sv!(b"manoeuvre"), [sv!(b"maneuver")]);
        add_rule!(special, sv!(b"manoeuvred"), [sv!(b"maneuvered")]);
        add_rule!(special, sv!(b"manoeuvres"), [sv!(b"maneuvers")]);
        add_rule!(special, sv!(b"manoeuvring"), [sv!(b"maneuvering")]);
        add_rule!(special, sv!(b"manoeuvrings"), [sv!(b"maneuverings")]);
        add_rule!(special, sv!(b"marginalisation"), [sv!(b"marginalization")]);
        add_rule!(special, sv!(b"marginalise"), [sv!(b"marginalize")]);
        add_rule!(special, sv!(b"marginalised"), [sv!(b"marginalized")]);
        add_rule!(special, sv!(b"marginalises"), [sv!(b"marginalizes")]);
        add_rule!(special, sv!(b"marginalising"), [sv!(b"marginalizing")]);
        add_rule!(special, sv!(b"marshalled"), [sv!(b"marshaled")]);
        add_rule!(special, sv!(b"marshalling"), [sv!(b"marshaling")]);
        add_rule!(special, sv!(b"marvelled"), [sv!(b"marveled")]);
        add_rule!(special, sv!(b"marvelling"), [sv!(b"marveling")]);
        add_rule!(special, sv!(b"marvellous"), [sv!(b"marvelous")]);
        add_rule!(special, sv!(b"marvellously"), [sv!(b"marvelously")]);
        add_rule!(special, sv!(b"materialisation"), [sv!(b"materialization")]);
        add_rule!(special, sv!(b"materialise"), [sv!(b"materialize")]);
        add_rule!(special, sv!(b"materialised"), [sv!(b"materialized")]);
        add_rule!(special, sv!(b"materialises"), [sv!(b"materializes")]);
        add_rule!(special, sv!(b"materialising"), [sv!(b"materializing")]);
        add_rule!(special, sv!(b"maximisation"), [sv!(b"maximization")]);
        add_rule!(special, sv!(b"maximise"), [sv!(b"maximize")]);
        add_rule!(special, sv!(b"maximised"), [sv!(b"maximized")]);
        add_rule!(special, sv!(b"maximises"), [sv!(b"maximizes")]);
        add_rule!(special, sv!(b"maximising"), [sv!(b"maximizing")]);
        add_rule!(special, sv!(b"meagre"), [sv!(b"meager")]);
        add_rule!(special, sv!(b"mechanisation"), [sv!(b"mechanization")]);
        add_rule!(special, sv!(b"mechanise"), [sv!(b"mechanize")]);
        add_rule!(special, sv!(b"mechanised"), [sv!(b"mechanized")]);
        add_rule!(special, sv!(b"mechanises"), [sv!(b"mechanizes")]);
        add_rule!(special, sv!(b"mechanising"), [sv!(b"mechanizing")]);
        add_rule!(special, sv!(b"mediaeval"), [sv!(b"medieval")]);
        add_rule!(special, sv!(b"memorialise"), [sv!(b"memorialize")]);
        add_rule!(special, sv!(b"memorialised"), [sv!(b"memorialized")]);
        add_rule!(special, sv!(b"memorialises"), [sv!(b"memorializes")]);
        add_rule!(special, sv!(b"memorialising"), [sv!(b"memorializing")]);
        add_rule!(special, sv!(b"memorise"), [sv!(b"memorize")]);
        add_rule!(special, sv!(b"memorised"), [sv!(b"memorized")]);
        add_rule!(special, sv!(b"memorises"), [sv!(b"memorizes")]);
        add_rule!(special, sv!(b"memorising"), [sv!(b"memorizing")]);
        add_rule!(special, sv!(b"mesmerise"), [sv!(b"mesmerize")]);
        add_rule!(special, sv!(b"mesmerised"), [sv!(b"mesmerized")]);
        add_rule!(special, sv!(b"mesmerises"), [sv!(b"mesmerizes")]);
        add_rule!(special, sv!(b"mesmerising"), [sv!(b"mesmerizing")]);
        add_rule!(special, sv!(b"metabolise"), [sv!(b"metabolize")]);
        add_rule!(special, sv!(b"metabolised"), [sv!(b"metabolized")]);
        add_rule!(special, sv!(b"metabolises"), [sv!(b"metabolizes")]);
        add_rule!(special, sv!(b"metabolising"), [sv!(b"metabolizing")]);
        add_rule!(special, sv!(b"metre"), [sv!(b"meter")]);
        add_rule!(special, sv!(b"metres"), [sv!(b"meters")]);
        add_rule!(special, sv!(b"micrometre"), [sv!(b"micrometer")]);
        add_rule!(special, sv!(b"micrometres"), [sv!(b"micrometers")]);
        add_rule!(special, sv!(b"militarise"), [sv!(b"militarize")]);
        add_rule!(special, sv!(b"militarised"), [sv!(b"militarized")]);
        add_rule!(special, sv!(b"militarises"), [sv!(b"militarizes")]);
        add_rule!(special, sv!(b"militarising"), [sv!(b"militarizing")]);
        add_rule!(special, sv!(b"milligramme"), [sv!(b"milligram")]);
        add_rule!(special, sv!(b"milligrammes"), [sv!(b"milligrams")]);
        add_rule!(special, sv!(b"millilitre"), [sv!(b"milliliter")]);
        add_rule!(special, sv!(b"millilitres"), [sv!(b"milliliters")]);
        add_rule!(special, sv!(b"millimetre"), [sv!(b"millimeter")]);
        add_rule!(special, sv!(b"millimetres"), [sv!(b"millimeters")]);
        add_rule!(special, sv!(b"miniaturisation"), [sv!(b"miniaturization")]);
        add_rule!(special, sv!(b"miniaturise"), [sv!(b"miniaturize")]);
        add_rule!(special, sv!(b"miniaturised"), [sv!(b"miniaturized")]);
        add_rule!(special, sv!(b"miniaturises"), [sv!(b"miniaturizes")]);
        add_rule!(special, sv!(b"miniaturising"), [sv!(b"miniaturizing")]);
        add_rule!(special, sv!(b"minibuses"), [sv!(b"minibusses ")]);
        add_rule!(special, sv!(b"minimise"), [sv!(b"minimize")]);
        add_rule!(special, sv!(b"minimised"), [sv!(b"minimized")]);
        add_rule!(special, sv!(b"minimises"), [sv!(b"minimizes")]);
        add_rule!(special, sv!(b"minimising"), [sv!(b"minimizing")]);
        add_rule!(special, sv!(b"misbehaviour"), [sv!(b"misbehavior")]);
        add_rule!(special, sv!(b"misdemeanour"), [sv!(b"misdemeanor")]);
        add_rule!(special, sv!(b"misdemeanours"), [sv!(b"misdemeanors")]);
        add_rule!(special, sv!(b"misspelt"), [sv!(b"misspelled ")]);
        add_rule!(special, sv!(b"mitre"), [sv!(b"miter")]);
        add_rule!(special, sv!(b"mitres"), [sv!(b"miters")]);
        add_rule!(special, sv!(b"mobilisation"), [sv!(b"mobilization")]);
        add_rule!(special, sv!(b"mobilise"), [sv!(b"mobilize")]);
        add_rule!(special, sv!(b"mobilised"), [sv!(b"mobilized")]);
        add_rule!(special, sv!(b"mobilises"), [sv!(b"mobilizes")]);
        add_rule!(special, sv!(b"mobilising"), [sv!(b"mobilizing")]);
        add_rule!(special, sv!(b"modelled"), [sv!(b"modeled")]);
        add_rule!(special, sv!(b"modeller"), [sv!(b"modeler")]);
        add_rule!(special, sv!(b"modellers"), [sv!(b"modelers")]);
        add_rule!(special, sv!(b"modelling"), [sv!(b"modeling")]);
        add_rule!(special, sv!(b"modernise"), [sv!(b"modernize")]);
        add_rule!(special, sv!(b"modernised"), [sv!(b"modernized")]);
        add_rule!(special, sv!(b"modernises"), [sv!(b"modernizes")]);
        add_rule!(special, sv!(b"modernising"), [sv!(b"modernizing")]);
        add_rule!(special, sv!(b"moisturise"), [sv!(b"moisturize")]);
        add_rule!(special, sv!(b"moisturised"), [sv!(b"moisturized")]);
        add_rule!(special, sv!(b"moisturiser"), [sv!(b"moisturizer")]);
        add_rule!(special, sv!(b"moisturisers"), [sv!(b"moisturizers")]);
        add_rule!(special, sv!(b"moisturises"), [sv!(b"moisturizes")]);
        add_rule!(special, sv!(b"moisturising"), [sv!(b"moisturizing")]);
        add_rule!(special, sv!(b"monologue"), [sv!(b"monolog")]);
        add_rule!(special, sv!(b"monologues"), [sv!(b"monologs")]);
        add_rule!(special, sv!(b"monopolisation"), [sv!(b"monopolization")]);
        add_rule!(special, sv!(b"monopolise"), [sv!(b"monopolize")]);
        add_rule!(special, sv!(b"monopolised"), [sv!(b"monopolized")]);
        add_rule!(special, sv!(b"monopolises"), [sv!(b"monopolizes")]);
        add_rule!(special, sv!(b"monopolising"), [sv!(b"monopolizing")]);
        add_rule!(special, sv!(b"moralise"), [sv!(b"moralize")]);
        add_rule!(special, sv!(b"moralised"), [sv!(b"moralized")]);
        add_rule!(special, sv!(b"moralises"), [sv!(b"moralizes")]);
        add_rule!(special, sv!(b"moralising"), [sv!(b"moralizing")]);
        add_rule!(special, sv!(b"motorised"), [sv!(b"motorized")]);
        add_rule!(special, sv!(b"mould"), [sv!(b"mold")]);
        add_rule!(special, sv!(b"moulded"), [sv!(b"molded")]);
        add_rule!(special, sv!(b"moulder"), [sv!(b"molder")]);
        add_rule!(special, sv!(b"mouldered"), [sv!(b"moldered")]);
        add_rule!(special, sv!(b"mouldering"), [sv!(b"moldering")]);
        add_rule!(special, sv!(b"moulders"), [sv!(b"molders")]);
        add_rule!(special, sv!(b"mouldier"), [sv!(b"moldier")]);
        add_rule!(special, sv!(b"mouldiest"), [sv!(b"moldiest")]);
        add_rule!(special, sv!(b"moulding"), [sv!(b"molding")]);
        add_rule!(special, sv!(b"mouldings"), [sv!(b"moldings")]);
        add_rule!(special, sv!(b"moulds"), [sv!(b"molds")]);
        add_rule!(special, sv!(b"mouldy"), [sv!(b"moldy")]);
        add_rule!(special, sv!(b"moult"), [sv!(b"molt")]);
        add_rule!(special, sv!(b"moulted"), [sv!(b"molted")]);
        add_rule!(special, sv!(b"moulting"), [sv!(b"molting")]);
        add_rule!(special, sv!(b"moults"), [sv!(b"molts")]);
        add_rule!(special, sv!(b"moustache"), [sv!(b"mustache")]);
        add_rule!(special, sv!(b"moustached"), [sv!(b"mustached")]);
        add_rule!(special, sv!(b"moustaches"), [sv!(b"mustaches")]);
        add_rule!(special, sv!(b"moustachioed"), [sv!(b"mustachioed")]);
        add_rule!(special, sv!(b"multicoloured"), [sv!(b"multicolored")]);
        add_rule!(special, sv!(b"nationalisation"), [sv!(b"nationalization")]);
        add_rule!(
            special,
            sv!(b"nationalisations"),
            [sv!(b"nationalizations")]
        );
        add_rule!(special, sv!(b"nationalise"), [sv!(b"nationalize")]);
        add_rule!(special, sv!(b"nationalised"), [sv!(b"nationalized")]);
        add_rule!(special, sv!(b"nationalises"), [sv!(b"nationalizes")]);
        add_rule!(special, sv!(b"nationalising"), [sv!(b"nationalizing")]);
        add_rule!(special, sv!(b"naturalisation"), [sv!(b"naturalization")]);
        add_rule!(special, sv!(b"naturalise"), [sv!(b"naturalize")]);
        add_rule!(special, sv!(b"naturalised"), [sv!(b"naturalized")]);
        add_rule!(special, sv!(b"naturalises"), [sv!(b"naturalizes")]);
        add_rule!(special, sv!(b"naturalising"), [sv!(b"naturalizing")]);
        add_rule!(special, sv!(b"neighbour"), [sv!(b"neighbor")]);
        add_rule!(special, sv!(b"neighbourhood"), [sv!(b"neighborhood")]);
        add_rule!(special, sv!(b"neighbourhoods"), [sv!(b"neighborhoods")]);
        add_rule!(special, sv!(b"neighbouring"), [sv!(b"neighboring")]);
        add_rule!(special, sv!(b"neighbourliness"), [sv!(b"neighborliness")]);
        add_rule!(special, sv!(b"neighbourly"), [sv!(b"neighborly")]);
        add_rule!(special, sv!(b"neighbours"), [sv!(b"neighbors")]);
        add_rule!(special, sv!(b"neutralisation"), [sv!(b"neutralization")]);
        add_rule!(special, sv!(b"neutralise"), [sv!(b"neutralize")]);
        add_rule!(special, sv!(b"neutralised"), [sv!(b"neutralized")]);
        add_rule!(special, sv!(b"neutralises"), [sv!(b"neutralizes")]);
        add_rule!(special, sv!(b"neutralising"), [sv!(b"neutralizing")]);
        add_rule!(special, sv!(b"normalisation"), [sv!(b"normalization")]);
        add_rule!(special, sv!(b"normalise"), [sv!(b"normalize")]);
        add_rule!(special, sv!(b"normalised"), [sv!(b"normalized")]);
        add_rule!(special, sv!(b"normalises"), [sv!(b"normalizes")]);
        add_rule!(special, sv!(b"normalising"), [sv!(b"normalizing")]);
        add_rule!(special, sv!(b"odour"), [sv!(b"odor")]);
        add_rule!(special, sv!(b"odourless"), [sv!(b"odorless")]);
        add_rule!(special, sv!(b"odours"), [sv!(b"odors")]);
        add_rule!(special, sv!(b"oesophagus"), [sv!(b"esophagus")]);
        add_rule!(special, sv!(b"oesophaguses"), [sv!(b"esophaguses")]);
        add_rule!(special, sv!(b"oestrogen"), [sv!(b"estrogen")]);
        add_rule!(special, sv!(b"offence"), [sv!(b"offense")]);
        add_rule!(special, sv!(b"offences"), [sv!(b"offenses")]);
        add_rule!(special, sv!(b"omelette"), [sv!(b"omelet")]);
        add_rule!(special, sv!(b"omelettes"), [sv!(b"omelets")]);
        add_rule!(special, sv!(b"optimise"), [sv!(b"optimize")]);
        add_rule!(special, sv!(b"optimised"), [sv!(b"optimized")]);
        add_rule!(special, sv!(b"optimises"), [sv!(b"optimizes")]);
        add_rule!(special, sv!(b"optimising"), [sv!(b"optimizing")]);
        add_rule!(special, sv!(b"organisation"), [sv!(b"organization")]);
        add_rule!(special, sv!(b"organisational"), [sv!(b"organizational")]);
        add_rule!(special, sv!(b"organisations"), [sv!(b"organizations")]);
        add_rule!(special, sv!(b"organise"), [sv!(b"organize")]);
        add_rule!(special, sv!(b"organised"), [sv!(b"organized")]);
        add_rule!(special, sv!(b"organiser"), [sv!(b"organizer")]);
        add_rule!(special, sv!(b"organisers"), [sv!(b"organizers")]);
        add_rule!(special, sv!(b"organises"), [sv!(b"organizes")]);
        add_rule!(special, sv!(b"organising"), [sv!(b"organizing")]);
        add_rule!(special, sv!(b"orthopaedic"), [sv!(b"orthopedic")]);
        add_rule!(special, sv!(b"orthopaedics"), [sv!(b"orthopedics")]);
        add_rule!(special, sv!(b"ostracise"), [sv!(b"ostracize")]);
        add_rule!(special, sv!(b"ostracised"), [sv!(b"ostracized")]);
        add_rule!(special, sv!(b"ostracises"), [sv!(b"ostracizes")]);
        add_rule!(special, sv!(b"ostracising"), [sv!(b"ostracizing")]);
        add_rule!(special, sv!(b"outmanoeuvre"), [sv!(b"outmaneuver")]);
        add_rule!(special, sv!(b"outmanoeuvred"), [sv!(b"outmaneuvered")]);
        add_rule!(special, sv!(b"outmanoeuvres"), [sv!(b"outmaneuvers")]);
        add_rule!(special, sv!(b"outmanoeuvring"), [sv!(b"outmaneuvering")]);
        add_rule!(special, sv!(b"overemphasise"), [sv!(b"overemphasize")]);
        add_rule!(special, sv!(b"overemphasised"), [sv!(b"overemphasized")]);
        add_rule!(special, sv!(b"overemphasises"), [sv!(b"overemphasizes")]);
        add_rule!(special, sv!(b"overemphasising"), [sv!(b"overemphasizing")]);
        add_rule!(special, sv!(b"oxidisation"), [sv!(b"oxidization")]);
        add_rule!(special, sv!(b"oxidise"), [sv!(b"oxidize")]);
        add_rule!(special, sv!(b"oxidised"), [sv!(b"oxidized")]);
        add_rule!(special, sv!(b"oxidises"), [sv!(b"oxidizes")]);
        add_rule!(special, sv!(b"oxidising"), [sv!(b"oxidizing")]);
        add_rule!(special, sv!(b"paederast"), [sv!(b"pederast")]);
        add_rule!(special, sv!(b"paederasts"), [sv!(b"pederasts")]);
        add_rule!(special, sv!(b"paediatric"), [sv!(b"pediatric")]);
        add_rule!(special, sv!(b"paediatrician"), [sv!(b"pediatrician")]);
        add_rule!(special, sv!(b"paediatricians"), [sv!(b"pediatricians")]);
        add_rule!(special, sv!(b"paediatrics"), [sv!(b"pediatrics")]);
        add_rule!(special, sv!(b"paedophile"), [sv!(b"pedophile")]);
        add_rule!(special, sv!(b"paedophiles"), [sv!(b"pedophiles")]);
        add_rule!(special, sv!(b"paedophilia"), [sv!(b"pedophilia")]);
        add_rule!(special, sv!(b"palaeolithic"), [sv!(b"paleolithic")]);
        add_rule!(special, sv!(b"palaeontologist"), [sv!(b"paleontologist")]);
        add_rule!(special, sv!(b"palaeontologists"), [sv!(b"paleontologists")]);
        add_rule!(special, sv!(b"palaeontology"), [sv!(b"paleontology")]);
        add_rule!(special, sv!(b"panelled"), [sv!(b"paneled")]);
        add_rule!(special, sv!(b"panelling"), [sv!(b"paneling")]);
        add_rule!(special, sv!(b"panellist"), [sv!(b"panelist")]);
        add_rule!(special, sv!(b"panellists"), [sv!(b"panelists")]);
        add_rule!(special, sv!(b"paralyse"), [sv!(b"paralyze")]);
        add_rule!(special, sv!(b"paralysed"), [sv!(b"paralyzed")]);
        add_rule!(special, sv!(b"paralyses"), [sv!(b"paralyzes")]);
        add_rule!(special, sv!(b"paralysing"), [sv!(b"paralyzing")]);
        add_rule!(special, sv!(b"parcelled"), [sv!(b"parceled")]);
        add_rule!(special, sv!(b"parcelling"), [sv!(b"parceling")]);
        add_rule!(special, sv!(b"parlour"), [sv!(b"parlor")]);
        add_rule!(special, sv!(b"parlours"), [sv!(b"parlors")]);
        add_rule!(special, sv!(b"particularise"), [sv!(b"particularize")]);
        add_rule!(special, sv!(b"particularised"), [sv!(b"particularized")]);
        add_rule!(special, sv!(b"particularises"), [sv!(b"particularizes")]);
        add_rule!(special, sv!(b"particularising"), [sv!(b"particularizing")]);
        add_rule!(special, sv!(b"passivisation"), [sv!(b"passivization")]);
        add_rule!(special, sv!(b"passivise"), [sv!(b"passivize")]);
        add_rule!(special, sv!(b"passivised"), [sv!(b"passivized")]);
        add_rule!(special, sv!(b"passivises"), [sv!(b"passivizes")]);
        add_rule!(special, sv!(b"passivising"), [sv!(b"passivizing")]);
        add_rule!(special, sv!(b"pasteurisation"), [sv!(b"pasteurization")]);
        add_rule!(special, sv!(b"pasteurise"), [sv!(b"pasteurize")]);
        add_rule!(special, sv!(b"pasteurised"), [sv!(b"pasteurized")]);
        add_rule!(special, sv!(b"pasteurises"), [sv!(b"pasteurizes")]);
        add_rule!(special, sv!(b"pasteurising"), [sv!(b"pasteurizing")]);
        add_rule!(special, sv!(b"patronise"), [sv!(b"patronize")]);
        add_rule!(special, sv!(b"patronised"), [sv!(b"patronized")]);
        add_rule!(special, sv!(b"patronises"), [sv!(b"patronizes")]);
        add_rule!(special, sv!(b"patronising"), [sv!(b"patronizing")]);
        add_rule!(special, sv!(b"patronisingly"), [sv!(b"patronizingly")]);
        add_rule!(special, sv!(b"pedalled"), [sv!(b"pedaled")]);
        add_rule!(special, sv!(b"pedalling"), [sv!(b"pedaling")]);
        add_rule!(
            special,
            sv!(b"pedestrianisation"),
            [sv!(b"pedestrianization")]
        );
        add_rule!(special, sv!(b"pedestrianise"), [sv!(b"pedestrianize")]);
        add_rule!(special, sv!(b"pedestrianised"), [sv!(b"pedestrianized")]);
        add_rule!(special, sv!(b"pedestrianises"), [sv!(b"pedestrianizes")]);
        add_rule!(special, sv!(b"pedestrianising"), [sv!(b"pedestrianizing")]);
        add_rule!(special, sv!(b"penalise"), [sv!(b"penalize")]);
        add_rule!(special, sv!(b"penalised"), [sv!(b"penalized")]);
        add_rule!(special, sv!(b"penalises"), [sv!(b"penalizes")]);
        add_rule!(special, sv!(b"penalising"), [sv!(b"penalizing")]);
        add_rule!(special, sv!(b"pencilled"), [sv!(b"penciled")]);
        add_rule!(special, sv!(b"pencilling"), [sv!(b"penciling")]);
        add_rule!(special, sv!(b"personalise"), [sv!(b"personalize")]);
        add_rule!(special, sv!(b"personalised"), [sv!(b"personalized")]);
        add_rule!(special, sv!(b"personalises"), [sv!(b"personalizes")]);
        add_rule!(special, sv!(b"personalising"), [sv!(b"personalizing")]);
        add_rule!(special, sv!(b"pharmacopoeia"), [sv!(b"pharmacopeia")]);
        add_rule!(special, sv!(b"pharmacopoeias"), [sv!(b"pharmacopeias")]);
        add_rule!(special, sv!(b"philosophise"), [sv!(b"philosophize")]);
        add_rule!(special, sv!(b"philosophised"), [sv!(b"philosophized")]);
        add_rule!(special, sv!(b"philosophises"), [sv!(b"philosophizes")]);
        add_rule!(special, sv!(b"philosophising"), [sv!(b"philosophizing")]);
        add_rule!(special, sv!(b"philtre"), [sv!(b"filter")]);
        add_rule!(special, sv!(b"philtres"), [sv!(b"filters")]);
        add_rule!(special, sv!(b"phoney "), [sv!(b"phony ")]);
        add_rule!(special, sv!(b"plagiarise"), [sv!(b"plagiarize")]);
        add_rule!(special, sv!(b"plagiarised"), [sv!(b"plagiarized")]);
        add_rule!(special, sv!(b"plagiarises"), [sv!(b"plagiarizes")]);
        add_rule!(special, sv!(b"plagiarising"), [sv!(b"plagiarizing")]);
        add_rule!(special, sv!(b"plough"), [sv!(b"plow")]);
        add_rule!(special, sv!(b"ploughed"), [sv!(b"plowed")]);
        add_rule!(special, sv!(b"ploughing"), [sv!(b"plowing")]);
        add_rule!(special, sv!(b"ploughman"), [sv!(b"plowman")]);
        add_rule!(special, sv!(b"ploughmen"), [sv!(b"plowmen")]);
        add_rule!(special, sv!(b"ploughs"), [sv!(b"plows")]);
        add_rule!(special, sv!(b"ploughshare"), [sv!(b"plowshare")]);
        add_rule!(special, sv!(b"ploughshares"), [sv!(b"plowshares")]);
        add_rule!(special, sv!(b"polarisation"), [sv!(b"polarization")]);
        add_rule!(special, sv!(b"polarise"), [sv!(b"polarize")]);
        add_rule!(special, sv!(b"polarised"), [sv!(b"polarized")]);
        add_rule!(special, sv!(b"polarises"), [sv!(b"polarizes")]);
        add_rule!(special, sv!(b"polarising"), [sv!(b"polarizing")]);
        add_rule!(special, sv!(b"politicisation"), [sv!(b"politicization")]);
        add_rule!(special, sv!(b"politicise"), [sv!(b"politicize")]);
        add_rule!(special, sv!(b"politicised"), [sv!(b"politicized")]);
        add_rule!(special, sv!(b"politicises"), [sv!(b"politicizes")]);
        add_rule!(special, sv!(b"politicising"), [sv!(b"politicizing")]);
        add_rule!(special, sv!(b"popularisation"), [sv!(b"popularization")]);
        add_rule!(special, sv!(b"popularise"), [sv!(b"popularize")]);
        add_rule!(special, sv!(b"popularised"), [sv!(b"popularized")]);
        add_rule!(special, sv!(b"popularises"), [sv!(b"popularizes")]);
        add_rule!(special, sv!(b"popularising"), [sv!(b"popularizing")]);
        add_rule!(special, sv!(b"pouffe"), [sv!(b"pouf")]);
        add_rule!(special, sv!(b"pouffes"), [sv!(b"poufs")]);
        add_rule!(special, sv!(b"practise"), [sv!(b"practice")]);
        add_rule!(special, sv!(b"practised"), [sv!(b"practiced")]);
        add_rule!(special, sv!(b"practises"), [sv!(b"practices")]);
        add_rule!(special, sv!(b"practising "), [sv!(b"practicing ")]);
        add_rule!(special, sv!(b"praesidium"), [sv!(b"presidium")]);
        add_rule!(special, sv!(b"praesidiums "), [sv!(b"presidiums ")]);
        add_rule!(special, sv!(b"pressurisation"), [sv!(b"pressurization")]);
        add_rule!(special, sv!(b"pressurise"), [sv!(b"pressurize")]);
        add_rule!(special, sv!(b"pressurised"), [sv!(b"pressurized")]);
        add_rule!(special, sv!(b"pressurises"), [sv!(b"pressurizes")]);
        add_rule!(special, sv!(b"pressurising"), [sv!(b"pressurizing")]);
        add_rule!(special, sv!(b"pretence"), [sv!(b"pretense")]);
        add_rule!(special, sv!(b"pretences"), [sv!(b"pretenses")]);
        add_rule!(special, sv!(b"primaeval"), [sv!(b"primeval")]);
        add_rule!(special, sv!(b"prioritisation"), [sv!(b"prioritization")]);
        add_rule!(special, sv!(b"prioritise"), [sv!(b"prioritize")]);
        add_rule!(special, sv!(b"prioritised"), [sv!(b"prioritized")]);
        add_rule!(special, sv!(b"prioritises"), [sv!(b"prioritizes")]);
        add_rule!(special, sv!(b"prioritising"), [sv!(b"prioritizing")]);
        add_rule!(special, sv!(b"privatisation"), [sv!(b"privatization")]);
        add_rule!(special, sv!(b"privatisations"), [sv!(b"privatizations")]);
        add_rule!(special, sv!(b"privatise"), [sv!(b"privatize")]);
        add_rule!(special, sv!(b"privatised"), [sv!(b"privatized")]);
        add_rule!(special, sv!(b"privatises"), [sv!(b"privatizes")]);
        add_rule!(special, sv!(b"privatising"), [sv!(b"privatizing")]);
        add_rule!(
            special,
            sv!(b"professionalisation"),
            [sv!(b"professionalization")]
        );
        add_rule!(special, sv!(b"professionalise"), [sv!(b"professionalize")]);
        add_rule!(
            special,
            sv!(b"professionalised"),
            [sv!(b"professionalized")]
        );
        add_rule!(
            special,
            sv!(b"professionalises"),
            [sv!(b"professionalizes")]
        );
        add_rule!(
            special,
            sv!(b"professionalising"),
            [sv!(b"professionalizing")]
        );
        add_rule!(special, sv!(b"programme"), [sv!(b"program")]);
        add_rule!(special, sv!(b"programmes"), [sv!(b"programs")]);
        add_rule!(special, sv!(b"prologue"), [sv!(b"prolog")]);
        add_rule!(special, sv!(b"prologues"), [sv!(b"prologs")]);
        add_rule!(special, sv!(b"propagandise"), [sv!(b"propagandize")]);
        add_rule!(special, sv!(b"propagandised"), [sv!(b"propagandized")]);
        add_rule!(special, sv!(b"propagandises"), [sv!(b"propagandizes")]);
        add_rule!(special, sv!(b"propagandising"), [sv!(b"propagandizing")]);
        add_rule!(special, sv!(b"proselytise"), [sv!(b"proselytize")]);
        add_rule!(special, sv!(b"proselytised"), [sv!(b"proselytized")]);
        add_rule!(special, sv!(b"proselytiser"), [sv!(b"proselytizer")]);
        add_rule!(special, sv!(b"proselytisers"), [sv!(b"proselytizers")]);
        add_rule!(special, sv!(b"proselytises"), [sv!(b"proselytizes")]);
        add_rule!(special, sv!(b"proselytising"), [sv!(b"proselytizing")]);
        add_rule!(special, sv!(b"psychoanalyse"), [sv!(b"psychoanalyze")]);
        add_rule!(special, sv!(b"psychoanalysed"), [sv!(b"psychoanalyzed")]);
        add_rule!(special, sv!(b"psychoanalyses"), [sv!(b"psychoanalyzes")]);
        add_rule!(special, sv!(b"psychoanalysing"), [sv!(b"psychoanalyzing")]);
        add_rule!(special, sv!(b"publicise"), [sv!(b"publicize")]);
        add_rule!(special, sv!(b"publicised"), [sv!(b"publicized")]);
        add_rule!(special, sv!(b"publicises"), [sv!(b"publicizes")]);
        add_rule!(special, sv!(b"publicising"), [sv!(b"publicizing")]);
        add_rule!(special, sv!(b"pulverisation"), [sv!(b"pulverization")]);
        add_rule!(special, sv!(b"pulverise"), [sv!(b"pulverize")]);
        add_rule!(special, sv!(b"pulverised"), [sv!(b"pulverized")]);
        add_rule!(special, sv!(b"pulverises"), [sv!(b"pulverizes")]);
        add_rule!(special, sv!(b"pulverising"), [sv!(b"pulverizing")]);
        add_rule!(special, sv!(b"pummelled"), [sv!(b"pummel")]);
        add_rule!(special, sv!(b"pummelling"), [sv!(b"pummeled")]);
        add_rule!(special, sv!(b"pyjama"), [sv!(b"pajama")]);
        add_rule!(special, sv!(b"pyjamas"), [sv!(b"pajamas")]);
        add_rule!(special, sv!(b"pzazz"), [sv!(b"pizzazz")]);
        add_rule!(special, sv!(b"quarrelled"), [sv!(b"quarreled")]);
        add_rule!(special, sv!(b"quarrelling"), [sv!(b"quarreling")]);
        add_rule!(special, sv!(b"radicalise"), [sv!(b"radicalize")]);
        add_rule!(special, sv!(b"radicalised"), [sv!(b"radicalized")]);
        add_rule!(special, sv!(b"radicalises"), [sv!(b"radicalizes")]);
        add_rule!(special, sv!(b"radicalising"), [sv!(b"radicalizing")]);
        add_rule!(special, sv!(b"rancour"), [sv!(b"rancor")]);
        add_rule!(special, sv!(b"randomise"), [sv!(b"randomize")]);
        add_rule!(special, sv!(b"randomised"), [sv!(b"randomized")]);
        add_rule!(special, sv!(b"randomises"), [sv!(b"randomizes")]);
        add_rule!(special, sv!(b"randomising"), [sv!(b"randomizing")]);
        add_rule!(special, sv!(b"rationalisation"), [sv!(b"rationalization")]);
        add_rule!(
            special,
            sv!(b"rationalisations"),
            [sv!(b"rationalizations")]
        );
        add_rule!(special, sv!(b"rationalise"), [sv!(b"rationalize")]);
        add_rule!(special, sv!(b"rationalised"), [sv!(b"rationalized")]);
        add_rule!(special, sv!(b"rationalises"), [sv!(b"rationalizes")]);
        add_rule!(special, sv!(b"rationalising"), [sv!(b"rationalizing")]);
        add_rule!(special, sv!(b"ravelled"), [sv!(b"raveled")]);
        add_rule!(special, sv!(b"ravelling"), [sv!(b"raveling")]);
        add_rule!(special, sv!(b"realisable"), [sv!(b"realizable")]);
        add_rule!(special, sv!(b"realisation"), [sv!(b"realization")]);
        add_rule!(special, sv!(b"realisations"), [sv!(b"realizations")]);
        add_rule!(special, sv!(b"realise"), [sv!(b"realize")]);
        add_rule!(special, sv!(b"realised"), [sv!(b"realized")]);
        add_rule!(special, sv!(b"realises"), [sv!(b"realizes")]);
        add_rule!(special, sv!(b"realising"), [sv!(b"realizing")]);
        add_rule!(special, sv!(b"recognisable"), [sv!(b"recognizable")]);
        add_rule!(special, sv!(b"recognisably"), [sv!(b"recognizably")]);
        add_rule!(special, sv!(b"recognisance"), [sv!(b"recognizance")]);
        add_rule!(special, sv!(b"recognise"), [sv!(b"recognize")]);
        add_rule!(special, sv!(b"recognised"), [sv!(b"recognized")]);
        add_rule!(special, sv!(b"recognises"), [sv!(b"recognizes")]);
        add_rule!(special, sv!(b"recognising"), [sv!(b"recognizing")]);
        add_rule!(special, sv!(b"reconnoitre"), [sv!(b"reconnoiter")]);
        add_rule!(special, sv!(b"reconnoitred"), [sv!(b"reconnoitered")]);
        add_rule!(special, sv!(b"reconnoitres"), [sv!(b"reconnoiters")]);
        add_rule!(special, sv!(b"reconnoitring"), [sv!(b"reconnoitering")]);
        add_rule!(special, sv!(b"refuelled"), [sv!(b"refueled")]);
        add_rule!(special, sv!(b"refuelling"), [sv!(b"refueling")]);
        add_rule!(special, sv!(b"regularisation"), [sv!(b"regularization")]);
        add_rule!(special, sv!(b"regularise"), [sv!(b"regularize")]);
        add_rule!(special, sv!(b"regularised"), [sv!(b"regularized")]);
        add_rule!(special, sv!(b"regularises"), [sv!(b"regularizes")]);
        add_rule!(special, sv!(b"regularising"), [sv!(b"regularizing")]);
        add_rule!(special, sv!(b"remodelled"), [sv!(b"remodeled")]);
        add_rule!(special, sv!(b"remodelling"), [sv!(b"remodeling")]);
        add_rule!(special, sv!(b"remould"), [sv!(b"remold")]);
        add_rule!(special, sv!(b"remoulded"), [sv!(b"remolded")]);
        add_rule!(special, sv!(b"remoulding"), [sv!(b"remolding")]);
        add_rule!(special, sv!(b"remoulds"), [sv!(b"remolds")]);
        add_rule!(special, sv!(b"reorganisation"), [sv!(b"reorganization")]);
        add_rule!(special, sv!(b"reorganisations"), [sv!(b"reorganizations")]);
        add_rule!(special, sv!(b"reorganise"), [sv!(b"reorganize")]);
        add_rule!(special, sv!(b"reorganised"), [sv!(b"reorganized")]);
        add_rule!(special, sv!(b"reorganises"), [sv!(b"reorganizes")]);
        add_rule!(special, sv!(b"reorganising"), [sv!(b"reorganizing")]);
        add_rule!(special, sv!(b"revelled"), [sv!(b"reveled")]);
        add_rule!(special, sv!(b"reveller"), [sv!(b"reveler")]);
        add_rule!(special, sv!(b"revellers"), [sv!(b"revelers")]);
        add_rule!(special, sv!(b"revelling"), [sv!(b"reveling")]);
        add_rule!(special, sv!(b"revitalise"), [sv!(b"revitalize")]);
        add_rule!(special, sv!(b"revitalised"), [sv!(b"revitalized")]);
        add_rule!(special, sv!(b"revitalises"), [sv!(b"revitalizes")]);
        add_rule!(special, sv!(b"revitalising"), [sv!(b"revitalizing")]);
        add_rule!(special, sv!(b"revolutionise"), [sv!(b"revolutionize")]);
        add_rule!(special, sv!(b"revolutionised"), [sv!(b"revolutionized")]);
        add_rule!(special, sv!(b"revolutionises"), [sv!(b"revolutionizes")]);
        add_rule!(special, sv!(b"revolutionising"), [sv!(b"revolutionizing")]);
        add_rule!(special, sv!(b"rhapsodise"), [sv!(b"rhapsodize")]);
        add_rule!(special, sv!(b"rhapsodised"), [sv!(b"rhapsodized")]);
        add_rule!(special, sv!(b"rhapsodises"), [sv!(b"rhapsodizes")]);
        add_rule!(special, sv!(b"rhapsodising"), [sv!(b"rhapsodizing")]);
        add_rule!(special, sv!(b"rigour"), [sv!(b"rigor")]);
        add_rule!(special, sv!(b"rigours"), [sv!(b"rigors")]);
        add_rule!(special, sv!(b"ritualised"), [sv!(b"ritualized")]);
        add_rule!(special, sv!(b"rivalled"), [sv!(b"rivaled")]);
        add_rule!(special, sv!(b"rivalling"), [sv!(b"rivaling")]);
        add_rule!(special, sv!(b"romanticise"), [sv!(b"romanticize")]);
        add_rule!(special, sv!(b"romanticised"), [sv!(b"romanticized")]);
        add_rule!(special, sv!(b"romanticises"), [sv!(b"romanticizes")]);
        add_rule!(special, sv!(b"romanticising"), [sv!(b"romanticizing")]);
        add_rule!(special, sv!(b"rumour"), [sv!(b"rumor")]);
        add_rule!(special, sv!(b"rumoured"), [sv!(b"rumored")]);
        add_rule!(special, sv!(b"rumours"), [sv!(b"rumors")]);
        add_rule!(special, sv!(b"sabre"), [sv!(b"saber")]);
        add_rule!(special, sv!(b"sabres"), [sv!(b"sabers")]);
        add_rule!(special, sv!(b"saltpetre"), [sv!(b"saltpeter")]);
        add_rule!(special, sv!(b"sanitise"), [sv!(b"sanitize")]);
        add_rule!(special, sv!(b"sanitised"), [sv!(b"sanitized")]);
        add_rule!(special, sv!(b"sanitises"), [sv!(b"sanitizes")]);
        add_rule!(special, sv!(b"sanitising"), [sv!(b"sanitizing")]);
        add_rule!(special, sv!(b"satirise"), [sv!(b"satirize")]);
        add_rule!(special, sv!(b"satirised"), [sv!(b"satirized")]);
        add_rule!(special, sv!(b"satirises"), [sv!(b"satirizes")]);
        add_rule!(special, sv!(b"satirising"), [sv!(b"satirizing")]);
        add_rule!(special, sv!(b"saviour"), [sv!(b"savior")]);
        add_rule!(special, sv!(b"saviours"), [sv!(b"saviors")]);
        add_rule!(special, sv!(b"savour"), [sv!(b"savor")]);
        add_rule!(special, sv!(b"savoured"), [sv!(b"savored")]);
        add_rule!(special, sv!(b"savouries"), [sv!(b"savories")]);
        add_rule!(special, sv!(b"savouring"), [sv!(b"savoring")]);
        add_rule!(special, sv!(b"savours"), [sv!(b"savors")]);
        add_rule!(special, sv!(b"savoury"), [sv!(b"savory")]);
        add_rule!(special, sv!(b"scandalise"), [sv!(b"scandalize")]);
        add_rule!(special, sv!(b"scandalised"), [sv!(b"scandalized")]);
        add_rule!(special, sv!(b"scandalises"), [sv!(b"scandalizes")]);
        add_rule!(special, sv!(b"scandalising"), [sv!(b"scandalizing")]);
        add_rule!(special, sv!(b"sceptic"), [sv!(b"skeptic")]);
        add_rule!(special, sv!(b"sceptical"), [sv!(b"skeptical")]);
        add_rule!(special, sv!(b"sceptically"), [sv!(b"skeptically")]);
        add_rule!(special, sv!(b"scepticism"), [sv!(b"skepticism")]);
        add_rule!(special, sv!(b"sceptics"), [sv!(b"skeptics")]);
        add_rule!(special, sv!(b"sceptre"), [sv!(b"scepter")]);
        add_rule!(special, sv!(b"sceptres"), [sv!(b"scepters")]);
        add_rule!(special, sv!(b"scrutinise"), [sv!(b"scrutinize")]);
        add_rule!(special, sv!(b"scrutinised"), [sv!(b"scrutinized")]);
        add_rule!(special, sv!(b"scrutinises"), [sv!(b"scrutinizes")]);
        add_rule!(special, sv!(b"scrutinising"), [sv!(b"scrutinizing")]);
        add_rule!(special, sv!(b"secularisation"), [sv!(b"secularization")]);
        add_rule!(special, sv!(b"secularise"), [sv!(b"secularize")]);
        add_rule!(special, sv!(b"secularised"), [sv!(b"secularized")]);
        add_rule!(special, sv!(b"secularises"), [sv!(b"secularizes")]);
        add_rule!(special, sv!(b"secularising"), [sv!(b"secularizing")]);
        add_rule!(special, sv!(b"sensationalise"), [sv!(b"sensationalize")]);
        add_rule!(special, sv!(b"sensationalised"), [sv!(b"sensationalized")]);
        add_rule!(special, sv!(b"sensationalises"), [sv!(b"sensationalizes")]);
        add_rule!(
            special,
            sv!(b"sensationalising"),
            [sv!(b"sensationalizing")]
        );
        add_rule!(special, sv!(b"sensitise"), [sv!(b"sensitize")]);
        add_rule!(special, sv!(b"sensitised"), [sv!(b"sensitized")]);
        add_rule!(special, sv!(b"sensitises"), [sv!(b"sensitizes")]);
        add_rule!(special, sv!(b"sensitising"), [sv!(b"sensitizing")]);
        add_rule!(special, sv!(b"sentimentalise"), [sv!(b"sentimentalize")]);
        add_rule!(special, sv!(b"sentimentalised"), [sv!(b"sentimentalized")]);
        add_rule!(special, sv!(b"sentimentalises"), [sv!(b"sentimentalizes")]);
        add_rule!(
            special,
            sv!(b"sentimentalising"),
            [sv!(b"sentimentalizing")]
        );
        add_rule!(special, sv!(b"sepulchre"), [sv!(b"sepulcher")]);
        add_rule!(special, sv!(b"sepulchres"), [sv!(b"sepulchers ")]);
        add_rule!(special, sv!(b"serialisation"), [sv!(b"serialization")]);
        add_rule!(special, sv!(b"serialisations"), [sv!(b"serializations")]);
        add_rule!(special, sv!(b"serialise"), [sv!(b"serialize")]);
        add_rule!(special, sv!(b"serialised"), [sv!(b"serialized")]);
        add_rule!(special, sv!(b"serialises"), [sv!(b"serializes")]);
        add_rule!(special, sv!(b"serialising"), [sv!(b"serializing")]);
        add_rule!(special, sv!(b"sermonise"), [sv!(b"sermonize")]);
        add_rule!(special, sv!(b"sermonised"), [sv!(b"sermonized")]);
        add_rule!(special, sv!(b"sermonises"), [sv!(b"sermonizes")]);
        add_rule!(special, sv!(b"sermonising"), [sv!(b"sermonizing")]);
        add_rule!(special, sv!(b"sheikh "), [sv!(b"sheik ")]);
        add_rule!(special, sv!(b"shovelled"), [sv!(b"shoveled")]);
        add_rule!(special, sv!(b"shovelling"), [sv!(b"shoveling")]);
        add_rule!(special, sv!(b"shrivelled"), [sv!(b"shriveled")]);
        add_rule!(special, sv!(b"shrivelling"), [sv!(b"shriveling")]);
        add_rule!(special, sv!(b"signalise"), [sv!(b"signalize")]);
        add_rule!(special, sv!(b"signalised"), [sv!(b"signalized")]);
        add_rule!(special, sv!(b"signalises"), [sv!(b"signalizes")]);
        add_rule!(special, sv!(b"signalising"), [sv!(b"signalizing")]);
        add_rule!(special, sv!(b"signalled"), [sv!(b"signaled")]);
        add_rule!(special, sv!(b"signalling"), [sv!(b"signaling")]);
        add_rule!(special, sv!(b"smoulder"), [sv!(b"smolder")]);
        add_rule!(special, sv!(b"smouldered"), [sv!(b"smoldered")]);
        add_rule!(special, sv!(b"smouldering"), [sv!(b"smoldering")]);
        add_rule!(special, sv!(b"smoulders"), [sv!(b"smolders")]);
        add_rule!(special, sv!(b"snivelled"), [sv!(b"sniveled")]);
        add_rule!(special, sv!(b"snivelling"), [sv!(b"sniveling")]);
        add_rule!(special, sv!(b"snorkelled"), [sv!(b"snorkeled")]);
        add_rule!(special, sv!(b"snorkelling"), [sv!(b"snorkeling")]);
        add_rule!(special, sv!(b"snowplough"), [sv!(b"snowplow")]);
        add_rule!(special, sv!(b"snowploughs"), [sv!(b"snowplow")]);
        add_rule!(special, sv!(b"socialisation"), [sv!(b"socialization")]);
        add_rule!(special, sv!(b"socialise"), [sv!(b"socialize")]);
        add_rule!(special, sv!(b"socialised"), [sv!(b"socialized")]);
        add_rule!(special, sv!(b"socialises"), [sv!(b"socializes")]);
        add_rule!(special, sv!(b"socialising"), [sv!(b"socializing")]);
        add_rule!(special, sv!(b"sodomise"), [sv!(b"sodomize")]);
        add_rule!(special, sv!(b"sodomised"), [sv!(b"sodomized")]);
        add_rule!(special, sv!(b"sodomises"), [sv!(b"sodomizes")]);
        add_rule!(special, sv!(b"sodomising"), [sv!(b"sodomizing")]);
        add_rule!(special, sv!(b"solemnise"), [sv!(b"solemnize")]);
        add_rule!(special, sv!(b"solemnised"), [sv!(b"solemnized")]);
        add_rule!(special, sv!(b"solemnises"), [sv!(b"solemnizes")]);
        add_rule!(special, sv!(b"solemnising"), [sv!(b"solemnizing")]);
        add_rule!(special, sv!(b"sombre"), [sv!(b"somber")]);
        add_rule!(special, sv!(b"specialisation"), [sv!(b"specialization")]);
        add_rule!(special, sv!(b"specialisations"), [sv!(b"specializations")]);
        add_rule!(special, sv!(b"specialise"), [sv!(b"specialize")]);
        add_rule!(special, sv!(b"specialised"), [sv!(b"specialized")]);
        add_rule!(special, sv!(b"specialises"), [sv!(b"specializes")]);
        add_rule!(special, sv!(b"specialising"), [sv!(b"specializing")]);
        add_rule!(special, sv!(b"spectre"), [sv!(b"specter")]);
        add_rule!(special, sv!(b"spectres"), [sv!(b"specters")]);
        add_rule!(special, sv!(b"spiralled"), [sv!(b"spiraled")]);
        add_rule!(special, sv!(b"spiralling"), [sv!(b"spiraling")]);
        add_rule!(special, sv!(b"splendour"), [sv!(b"splendor")]);
        add_rule!(special, sv!(b"splendours"), [sv!(b"splendors")]);
        add_rule!(special, sv!(b"squirrelled"), [sv!(b"squirreled")]);
        add_rule!(special, sv!(b"squirrelling"), [sv!(b"squirreling")]);
        add_rule!(special, sv!(b"stabilisation"), [sv!(b"stabilization")]);
        add_rule!(special, sv!(b"stabilise"), [sv!(b"stabilize")]);
        add_rule!(special, sv!(b"stabilised"), [sv!(b"stabilized")]);
        add_rule!(special, sv!(b"stabiliser"), [sv!(b"stabilizer")]);
        add_rule!(special, sv!(b"stabilisers"), [sv!(b"stabilizers")]);
        add_rule!(special, sv!(b"stabilises"), [sv!(b"stabilizes")]);
        add_rule!(special, sv!(b"stabilising"), [sv!(b"stabilizing")]);
        add_rule!(special, sv!(b"standardisation"), [sv!(b"standardization")]);
        add_rule!(special, sv!(b"standardise"), [sv!(b"standardize")]);
        add_rule!(special, sv!(b"standardised"), [sv!(b"standardized")]);
        add_rule!(special, sv!(b"standardises"), [sv!(b"standardizes")]);
        add_rule!(special, sv!(b"standardising"), [sv!(b"standardizing")]);
        add_rule!(special, sv!(b"stencilled"), [sv!(b"stenciled")]);
        add_rule!(special, sv!(b"stencilling"), [sv!(b"stenciling")]);
        add_rule!(special, sv!(b"sterilisation"), [sv!(b"sterilization")]);
        add_rule!(special, sv!(b"sterilisations"), [sv!(b"sterilizations")]);
        add_rule!(special, sv!(b"sterilise"), [sv!(b"sterilize")]);
        add_rule!(special, sv!(b"sterilised"), [sv!(b"sterilized")]);
        add_rule!(special, sv!(b"steriliser"), [sv!(b"sterilizer")]);
        add_rule!(special, sv!(b"sterilisers"), [sv!(b"sterilizers")]);
        add_rule!(special, sv!(b"sterilises"), [sv!(b"sterilizes")]);
        add_rule!(special, sv!(b"sterilising"), [sv!(b"sterilizing")]);
        add_rule!(special, sv!(b"stigmatisation"), [sv!(b"stigmatization")]);
        add_rule!(special, sv!(b"stigmatise"), [sv!(b"stigmatize")]);
        add_rule!(special, sv!(b"stigmatised"), [sv!(b"stigmatized")]);
        add_rule!(special, sv!(b"stigmatises"), [sv!(b"stigmatizes")]);
        add_rule!(special, sv!(b"stigmatising"), [sv!(b"stigmatizing")]);
        add_rule!(special, sv!(b"storey"), [sv!(b"story")]);
        add_rule!(special, sv!(b"storeys"), [sv!(b"stories")]);
        add_rule!(special, sv!(b"subsidisation"), [sv!(b"subsidization")]);
        add_rule!(special, sv!(b"subsidise"), [sv!(b"subsidize")]);
        add_rule!(special, sv!(b"subsidised"), [sv!(b"subsidized")]);
        add_rule!(special, sv!(b"subsidiser"), [sv!(b"subsidizer")]);
        add_rule!(special, sv!(b"subsidisers"), [sv!(b"subsidizers")]);
        add_rule!(special, sv!(b"subsidises"), [sv!(b"subsidizes")]);
        add_rule!(special, sv!(b"subsidising"), [sv!(b"subsidizing")]);
        add_rule!(special, sv!(b"succour"), [sv!(b"succor")]);
        add_rule!(special, sv!(b"succoured"), [sv!(b"succored")]);
        add_rule!(special, sv!(b"succouring"), [sv!(b"succoring")]);
        add_rule!(special, sv!(b"succours"), [sv!(b"succors")]);
        add_rule!(special, sv!(b"sulphate"), [sv!(b"sulfate")]);
        add_rule!(special, sv!(b"sulphates"), [sv!(b"sulfates")]);
        add_rule!(special, sv!(b"sulphide"), [sv!(b"sulfide")]);
        add_rule!(special, sv!(b"sulphides"), [sv!(b"sulfides")]);
        add_rule!(special, sv!(b"sulphur"), [sv!(b"sulfur")]);
        add_rule!(special, sv!(b"sulphurous"), [sv!(b"sulfurous")]);
        add_rule!(special, sv!(b"summarise"), [sv!(b"summarize")]);
        add_rule!(special, sv!(b"summarised"), [sv!(b"summarized")]);
        add_rule!(special, sv!(b"summarises"), [sv!(b"summarizes")]);
        add_rule!(special, sv!(b"summarising"), [sv!(b"summarizing")]);
        add_rule!(special, sv!(b"swivelled"), [sv!(b"swiveled")]);
        add_rule!(special, sv!(b"swivelling"), [sv!(b"swiveling")]);
        add_rule!(special, sv!(b"symbolise"), [sv!(b"symbolize")]);
        add_rule!(special, sv!(b"symbolised"), [sv!(b"symbolized")]);
        add_rule!(special, sv!(b"symbolises"), [sv!(b"symbolizes")]);
        add_rule!(special, sv!(b"symbolising"), [sv!(b"symbolizing")]);
        add_rule!(special, sv!(b"sympathise"), [sv!(b"sympathize")]);
        add_rule!(special, sv!(b"sympathised"), [sv!(b"sympathized")]);
        add_rule!(special, sv!(b"sympathiser"), [sv!(b"sympathizer")]);
        add_rule!(special, sv!(b"sympathisers"), [sv!(b"sympathizers")]);
        add_rule!(special, sv!(b"sympathises"), [sv!(b"sympathizes")]);
        add_rule!(special, sv!(b"sympathising"), [sv!(b"sympathizing")]);
        add_rule!(special, sv!(b"synchronisation"), [sv!(b"synchronization")]);
        add_rule!(special, sv!(b"synchronise"), [sv!(b"synchronize")]);
        add_rule!(special, sv!(b"synchronised"), [sv!(b"synchronized")]);
        add_rule!(special, sv!(b"synchronises"), [sv!(b"synchronizes")]);
        add_rule!(special, sv!(b"synchronising"), [sv!(b"synchronizing")]);
        add_rule!(special, sv!(b"synthesise"), [sv!(b"synthesize")]);
        add_rule!(special, sv!(b"synthesised"), [sv!(b"synthesized")]);
        add_rule!(special, sv!(b"synthesiser"), [sv!(b"synthesizer")]);
        add_rule!(special, sv!(b"synthesisers"), [sv!(b"synthesizers")]);
        add_rule!(special, sv!(b"synthesises"), [sv!(b"synthesizes")]);
        add_rule!(special, sv!(b"synthesising"), [sv!(b"synthesizing")]);
        add_rule!(special, sv!(b"syphon"), [sv!(b"siphon")]);
        add_rule!(special, sv!(b"syphoned"), [sv!(b"siphoned")]);
        add_rule!(special, sv!(b"syphoning"), [sv!(b"siphoning")]);
        add_rule!(special, sv!(b"syphons"), [sv!(b"siphons")]);
        add_rule!(special, sv!(b"systematisation"), [sv!(b"systematization")]);
        add_rule!(special, sv!(b"systematise"), [sv!(b"systematize")]);
        add_rule!(special, sv!(b"systematised"), [sv!(b"systematized")]);
        add_rule!(special, sv!(b"systematises"), [sv!(b"systematizes")]);
        add_rule!(special, sv!(b"systematising"), [sv!(b"systematizing")]);
        add_rule!(special, sv!(b"tantalise"), [sv!(b"tantalize")]);
        add_rule!(special, sv!(b"tantalised"), [sv!(b"tantalized")]);
        add_rule!(special, sv!(b"tantalises"), [sv!(b"tantalizes")]);
        add_rule!(special, sv!(b"tantalising"), [sv!(b"tantalizing")]);
        add_rule!(special, sv!(b"tantalisingly"), [sv!(b"tantalizingly")]);
        add_rule!(special, sv!(b"tasselled"), [sv!(b"tasseled")]);
        add_rule!(special, sv!(b"technicolour"), [sv!(b"technicolor")]);
        add_rule!(special, sv!(b"temporise"), [sv!(b"temporize")]);
        add_rule!(special, sv!(b"temporised"), [sv!(b"temporized")]);
        add_rule!(special, sv!(b"temporises"), [sv!(b"temporizes")]);
        add_rule!(special, sv!(b"temporising"), [sv!(b"temporizing")]);
        add_rule!(special, sv!(b"tenderise"), [sv!(b"tenderize")]);
        add_rule!(special, sv!(b"tenderised"), [sv!(b"tenderized")]);
        add_rule!(special, sv!(b"tenderises"), [sv!(b"tenderizes")]);
        add_rule!(special, sv!(b"tenderising"), [sv!(b"tenderizing")]);
        add_rule!(special, sv!(b"terrorise"), [sv!(b"terrorize")]);
        add_rule!(special, sv!(b"terrorised"), [sv!(b"terrorized")]);
        add_rule!(special, sv!(b"terrorises"), [sv!(b"terrorizes")]);
        add_rule!(special, sv!(b"terrorising"), [sv!(b"terrorizing")]);
        add_rule!(special, sv!(b"theatre"), [sv!(b"theater")]);
        add_rule!(special, sv!(b"theatregoer"), [sv!(b"theatergoer")]);
        add_rule!(special, sv!(b"theatregoers"), [sv!(b"theatergoers")]);
        add_rule!(special, sv!(b"theatres"), [sv!(b"theaters")]);
        add_rule!(special, sv!(b"theorise"), [sv!(b"theorize")]);
        add_rule!(special, sv!(b"theorised"), [sv!(b"theorized")]);
        add_rule!(special, sv!(b"theorises"), [sv!(b"theorizes")]);
        add_rule!(special, sv!(b"theorising"), [sv!(b"theorizing")]);
        add_rule!(special, sv!(b"tonne"), [sv!(b"ton")]);
        add_rule!(special, sv!(b"tonnes"), [sv!(b"tons")]);
        add_rule!(special, sv!(b"towelled"), [sv!(b"toweled")]);
        add_rule!(special, sv!(b"towelling"), [sv!(b"toweling")]);
        add_rule!(special, sv!(b"toxaemia"), [sv!(b"toxemia")]);
        add_rule!(special, sv!(b"tranquillise"), [sv!(b"tranquilize")]);
        add_rule!(special, sv!(b"tranquillised"), [sv!(b"tranquilized")]);
        add_rule!(special, sv!(b"tranquilliser"), [sv!(b"tranquilizer")]);
        add_rule!(special, sv!(b"tranquillisers"), [sv!(b"tranquilizers")]);
        add_rule!(special, sv!(b"tranquillises"), [sv!(b"tranquilizes")]);
        add_rule!(special, sv!(b"tranquillising"), [sv!(b"tranquilizing")]);
        add_rule!(special, sv!(b"tranquillity"), [sv!(b"tranquility")]);
        add_rule!(special, sv!(b"tranquillize"), [sv!(b"tranquilize")]);
        add_rule!(special, sv!(b"tranquillized"), [sv!(b"tranquilized")]);
        add_rule!(special, sv!(b"tranquillizer"), [sv!(b"tranquilizer")]);
        add_rule!(special, sv!(b"tranquillizers"), [sv!(b"tranquilizers")]);
        add_rule!(special, sv!(b"tranquillizes"), [sv!(b"tranquilizes")]);
        add_rule!(special, sv!(b"tranquillizing"), [sv!(b"tranquilizing")]);
        add_rule!(special, sv!(b"tranquilly"), [sv!(b"tranquility")]);
        add_rule!(special, sv!(b"transistorised"), [sv!(b"transistorized")]);
        add_rule!(special, sv!(b"traumatise"), [sv!(b"traumatize")]);
        add_rule!(special, sv!(b"traumatised"), [sv!(b"traumatized")]);
        add_rule!(special, sv!(b"traumatises"), [sv!(b"traumatizes")]);
        add_rule!(special, sv!(b"traumatising"), [sv!(b"traumatizing")]);
        add_rule!(special, sv!(b"travelled"), [sv!(b"traveled")]);
        add_rule!(special, sv!(b"traveller"), [sv!(b"traveler")]);
        add_rule!(special, sv!(b"travellers"), [sv!(b"travelers")]);
        add_rule!(special, sv!(b"travelling"), [sv!(b"traveling")]);
        add_rule!(special, sv!(b"travelogue"), [sv!(b"travelog")]);
        add_rule!(special, sv!(b"travelogues "), [sv!(b"travelogs ")]);
        add_rule!(special, sv!(b"trialled"), [sv!(b"trialed")]);
        add_rule!(special, sv!(b"trialling"), [sv!(b"trialing")]);
        add_rule!(special, sv!(b"tricolour"), [sv!(b"tricolor")]);
        add_rule!(special, sv!(b"tricolours"), [sv!(b"tricolors")]);
        add_rule!(special, sv!(b"trivialise"), [sv!(b"trivialize")]);
        add_rule!(special, sv!(b"trivialised"), [sv!(b"trivialized")]);
        add_rule!(special, sv!(b"trivialises"), [sv!(b"trivializes")]);
        add_rule!(special, sv!(b"trivialising"), [sv!(b"trivializing")]);
        add_rule!(special, sv!(b"tumour"), [sv!(b"tumor")]);
        add_rule!(special, sv!(b"tumours"), [sv!(b"tumors")]);
        add_rule!(special, sv!(b"tunnelled"), [sv!(b"tunneled")]);
        add_rule!(special, sv!(b"tunnelling"), [sv!(b"tunneling")]);
        add_rule!(special, sv!(b"tyrannise"), [sv!(b"tyrannize")]);
        add_rule!(special, sv!(b"tyrannised"), [sv!(b"tyrannized")]);
        add_rule!(special, sv!(b"tyrannises"), [sv!(b"tyrannizes")]);
        add_rule!(special, sv!(b"tyrannising"), [sv!(b"tyrannizing")]);
        add_rule!(special, sv!(b"tyre"), [sv!(b"tire")]);
        add_rule!(special, sv!(b"tyres"), [sv!(b"tires")]);
        add_rule!(special, sv!(b"unauthorised"), [sv!(b"unauthorized")]);
        add_rule!(special, sv!(b"uncivilised"), [sv!(b"uncivilized")]);
        add_rule!(special, sv!(b"underutilised"), [sv!(b"underutilized")]);
        add_rule!(special, sv!(b"unequalled"), [sv!(b"unequaled")]);
        add_rule!(special, sv!(b"unfavourable"), [sv!(b"unfavorable")]);
        add_rule!(special, sv!(b"unfavourably"), [sv!(b"unfavorably")]);
        add_rule!(special, sv!(b"unionisation"), [sv!(b"unionization")]);
        add_rule!(special, sv!(b"unionise"), [sv!(b"unionize")]);
        add_rule!(special, sv!(b"unionised"), [sv!(b"unionized")]);
        add_rule!(special, sv!(b"unionises"), [sv!(b"unionizes")]);
        add_rule!(special, sv!(b"unionising"), [sv!(b"unionizing")]);
        add_rule!(special, sv!(b"unorganised"), [sv!(b"unorganized")]);
        add_rule!(special, sv!(b"unravelled"), [sv!(b"unraveled")]);
        add_rule!(special, sv!(b"unravelling"), [sv!(b"unraveling")]);
        add_rule!(special, sv!(b"unrecognisable"), [sv!(b"unrecognizable")]);
        add_rule!(special, sv!(b"unrecognised"), [sv!(b"unrecognized")]);
        add_rule!(special, sv!(b"unrivalled"), [sv!(b"unrivaled")]);
        add_rule!(special, sv!(b"unsavoury"), [sv!(b"unsavory")]);
        add_rule!(special, sv!(b"untrammelled"), [sv!(b"untrammeled")]);
        add_rule!(special, sv!(b"urbanisation"), [sv!(b"urbanization")]);
        add_rule!(special, sv!(b"urbanise"), [sv!(b"urbanize")]);
        add_rule!(special, sv!(b"urbanised"), [sv!(b"urbanized")]);
        add_rule!(special, sv!(b"urbanises"), [sv!(b"urbanizes")]);
        add_rule!(special, sv!(b"urbanising"), [sv!(b"urbanizing")]);
        add_rule!(special, sv!(b"utilisable"), [sv!(b"utilizable")]);
        add_rule!(special, sv!(b"utilisation"), [sv!(b"utilization")]);
        add_rule!(special, sv!(b"utilise"), [sv!(b"utilize")]);
        add_rule!(special, sv!(b"utilised"), [sv!(b"utilized")]);
        add_rule!(special, sv!(b"utilises"), [sv!(b"utilizes")]);
        add_rule!(special, sv!(b"utilising"), [sv!(b"utilizing")]);
        add_rule!(special, sv!(b"valour"), [sv!(b"valor")]);
        add_rule!(special, sv!(b"vandalise"), [sv!(b"vandalize")]);
        add_rule!(special, sv!(b"vandalised"), [sv!(b"vandalized")]);
        add_rule!(special, sv!(b"vandalises"), [sv!(b"vandalizes")]);
        add_rule!(special, sv!(b"vandalising"), [sv!(b"vandalizing")]);
        add_rule!(special, sv!(b"vaporisation"), [sv!(b"vaporization")]);
        add_rule!(special, sv!(b"vaporise"), [sv!(b"vaporize")]);
        add_rule!(special, sv!(b"vaporised"), [sv!(b"vaporized")]);
        add_rule!(special, sv!(b"vaporises"), [sv!(b"vaporizes")]);
        add_rule!(special, sv!(b"vaporising"), [sv!(b"vaporizing")]);
        add_rule!(special, sv!(b"vapour"), [sv!(b"vapor")]);
        add_rule!(special, sv!(b"vapours"), [sv!(b"vapors")]);
        add_rule!(special, sv!(b"verbalise"), [sv!(b"verbalize")]);
        add_rule!(special, sv!(b"verbalised"), [sv!(b"verbalized")]);
        add_rule!(special, sv!(b"verbalises"), [sv!(b"verbalizes")]);
        add_rule!(special, sv!(b"verbalising"), [sv!(b"verbalizing")]);
        add_rule!(special, sv!(b"victimisation"), [sv!(b"victimization")]);
        add_rule!(special, sv!(b"victimise"), [sv!(b"victimize")]);
        add_rule!(special, sv!(b"victimised"), [sv!(b"victimized")]);
        add_rule!(special, sv!(b"victimises"), [sv!(b"victimizes")]);
        add_rule!(special, sv!(b"victimising"), [sv!(b"victimizing")]);
        add_rule!(special, sv!(b"videodisc"), [sv!(b"videodisk")]);
        add_rule!(special, sv!(b"videodiscs"), [sv!(b"videodisks")]);
        add_rule!(special, sv!(b"vigour"), [sv!(b"vigor")]);
        add_rule!(special, sv!(b"visualisation"), [sv!(b"visualization")]);
        add_rule!(special, sv!(b"visualisations"), [sv!(b"visualizations")]);
        add_rule!(special, sv!(b"visualise"), [sv!(b"visualize")]);
        add_rule!(special, sv!(b"visualised"), [sv!(b"visualized")]);
        add_rule!(special, sv!(b"visualises"), [sv!(b"visualizes")]);
        add_rule!(special, sv!(b"visualising"), [sv!(b"visualizing")]);
        add_rule!(special, sv!(b"vocalisation"), [sv!(b"vocalization")]);
        add_rule!(special, sv!(b"vocalisations"), [sv!(b"vocalizations")]);
        add_rule!(special, sv!(b"vocalise"), [sv!(b"vocalize")]);
        add_rule!(special, sv!(b"vocalised"), [sv!(b"vocalized")]);
        add_rule!(special, sv!(b"vocalises"), [sv!(b"vocalizes")]);
        add_rule!(special, sv!(b"vocalising"), [sv!(b"vocalizing")]);
        add_rule!(special, sv!(b"vulcanised"), [sv!(b"vulcanized")]);
        add_rule!(special, sv!(b"vulgarisation"), [sv!(b"vulgarization")]);
        add_rule!(special, sv!(b"vulgarise"), [sv!(b"vulgarize")]);
        add_rule!(special, sv!(b"vulgarised"), [sv!(b"vulgarized")]);
        add_rule!(special, sv!(b"vulgarises"), [sv!(b"vulgarizes")]);
        add_rule!(special, sv!(b"vulgarising"), [sv!(b"vulgarizing")]);
        add_rule!(special, sv!(b"waggon"), [sv!(b"wagon")]);
        add_rule!(special, sv!(b"waggons"), [sv!(b"wagons")]);
        add_rule!(special, sv!(b"watercolour"), [sv!(b"watercolor")]);
        add_rule!(special, sv!(b"watercolours"), [sv!(b"watercolors")]);
        add_rule!(special, sv!(b"weaselled"), [sv!(b"weaseled")]);
        add_rule!(special, sv!(b"weaselling"), [sv!(b"weaseling")]);
        add_rule!(special, sv!(b"westernisation"), [sv!(b"westernization")]);
        add_rule!(special, sv!(b"westernise"), [sv!(b"westernize")]);
        add_rule!(special, sv!(b"westernised"), [sv!(b"westernized")]);
        add_rule!(special, sv!(b"westernises"), [sv!(b"westernizes")]);
        add_rule!(special, sv!(b"westernising"), [sv!(b"westernizing")]);
        add_rule!(special, sv!(b"womanise"), [sv!(b"womanize")]);
        add_rule!(special, sv!(b"womanised"), [sv!(b"womanized")]);
        add_rule!(special, sv!(b"womaniser"), [sv!(b"womanizer")]);
        add_rule!(special, sv!(b"womanisers"), [sv!(b"womanizers")]);
        add_rule!(special, sv!(b"womanises"), [sv!(b"womanizes")]);
        add_rule!(special, sv!(b"womanising"), [sv!(b"womanizing")]);
        add_rule!(special, sv!(b"woollen"), [sv!(b"woolen")]);
        add_rule!(special, sv!(b"woollens"), [sv!(b"woolens")]);
        add_rule!(special, sv!(b"woollies"), [sv!(b"woolies")]);
        add_rule!(special, sv!(b"woolly"), [sv!(b"wooly")]);
        add_rule!(special, sv!(b"worshipped "), [sv!(b"worshiped")]);
        add_rule!(special, sv!(b"worshipping "), [sv!(b"worshiping ")]);
        add_rule!(special, sv!(b"worshipper"), [sv!(b"worshiper")]);
        add_rule!(special, sv!(b"yodelled"), [sv!(b"yodeled")]);
        add_rule!(special, sv!(b"yodelling"), [sv!(b"yodeling")]);
        add_rule!(special, sv!(b"yoghourt"), [sv!(b"yogurt")]);
        add_rule!(special, sv!(b"yoghourts"), [sv!(b"yogurts")]);
        add_rule!(special, sv!(b"yoghurt"), [sv!(b"yogurt")]);
        add_rule!(special, sv!(b"yoghurts"), [sv!(b"yogurts")]);

        RuleSet {
            general_prefix: general_prefixes,
            general_suffix: general_suffixes,
            special_expand: special,
        }
    }

    // If there is an exact match between this string and a special expand,
    // We create a set of lexemes with canonicals and the text
    pub fn special_expand<'doc>(&self, string: &'doc [u8]) -> Option<Vec<Lexeme>> {
        let ret = self.special_expand.get(string);
        if let Some(vs) = ret {
            Some(
                vs.iter()
                    .map(|canonical| Lexeme {
                        value: canonical.clone(),
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
        string: &'doc [u8],
    ) -> Option<(Lexeme, &'doc [u8])> {
        for i in (1..string.len()).rev() {
            if let Some(prefix_canonical) = self.general_prefix.get(&string[..i]) {
                return Some((
                    Lexeme {
                        value: prefix_canonical.clone(),
                    },
                    &string[i..],
                ));
            }
        }
        None
    }

    // Matches the longest suffix
    pub fn general_suffix_remainder<'doc>(
        &self,
        string: &'doc [u8],
    ) -> Option<(Lexeme, &'doc [u8])> {
        for i in 1..string.len() {
            if let Some(suffix_canonical) = self.general_suffix.get(&string[i..]) {
                return Some((
                    Lexeme {
                        value: suffix_canonical.clone(),
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

        for s in unidecode(&string).to_ascii_lowercase().split_whitespace() {
            let mut substr = s.as_bytes();
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
                    value: Vec::from(substr),
                });
                break;
            }
        }
        lexemes
    }
}
