use bimap::BiMap;

pub struct VocabIndex(value);

pub struct Vocab {
    // use ascii lowercase
    counter:u64,
    values:BiMap<u64, Vec<u8>>,
}

impl Vocab {
    pub fn new() -> Vocab {
        Vocab {
            values: BiMap::new()
        }
    }

    pub fn add(&mut self, value:Vec<u8>) -> Option<VocabIndex> {
        match(self.values.insert_no_overwrite(self.counter, value)) {
            Ok(()) => {
                self.counter += 1;
                VocabIndex(self.counter)
            },
            Err(_, _) => None
        }
    }

    pub fn get(&self, value:VocabIndex) -> Option<&Vec<u8>> {
        self.values.get_by_left(value.0)
    }

    pub fn contains(&self, value: VocabIndex) -> bool {
        self.values.contains_by_left(value.0)
    }

    pub fn get_id_by_str(&self, value:&Vec<u8>) -> VocabIndex {
        VocabIndex(self.values.get_by_right(value))
    }

    pub fn contains_by_str(&self, value: &Vec<u8>) ->bool {
        self.values.contains_by_left(value)
    }
}
