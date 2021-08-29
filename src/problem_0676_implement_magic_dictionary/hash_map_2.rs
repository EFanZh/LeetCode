// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, HashSet};

type KeyMap = HashMap<Box<[u8]>, Vec<HashSet<u8>>>;

pub struct MagicDictionary {
    keys: HashMap<usize, KeyMap>,
}

impl MagicDictionary {
    fn new() -> Self {
        Self { keys: HashMap::new() }
    }

    fn vec_get_or_default<T: Default>(items: &mut Vec<T>, index: usize) -> &mut T {
        if items.len() < index + 1 {
            items.resize_with(index + 1, T::default);
        }

        &mut items[index]
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for word in dictionary {
            let word = word.into_bytes();
            let keys = self.keys.entry(word.len()).or_default();

            for (i, &c) in word.iter().enumerate() {
                let mut key = Vec::with_capacity(word.len() - 1);

                key.extend(&word[..i]);
                key.extend(&word[i + 1..]);

                Self::vec_get_or_default(&mut keys.entry(key.into_boxed_slice()).or_default(), i).insert(c);
            }
        }
    }

    fn search(&self, search_word: String) -> bool {
        self.keys.get(&search_word.len()).map_or(false, |keys| {
            let search_word = search_word.into_bytes();
            let mut buffer = search_word[1..].to_vec().into_boxed_slice();

            for (i, c) in search_word.iter().enumerate() {
                if let Some(removed) = keys.get(&buffer).and_then(|removed| removed.get(i)) {
                    if removed.contains(c) {
                        if removed.len() > 1 {
                            return true;
                        }
                    } else if !removed.is_empty() {
                        return true;
                    }
                }

                if let Some(slot) = buffer.get_mut(i) {
                    *slot = *c;
                }
            }

            false
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MagicDictionary for MagicDictionary {
    fn new() -> Self {
        Self::new()
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        self.build_dict(dictionary);
    }

    fn search(&self, search_word: String) -> bool {
        self.search(search_word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MagicDictionary>();
    }
}
