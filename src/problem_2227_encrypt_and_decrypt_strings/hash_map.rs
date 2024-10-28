// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Encrypter {
    encryption_map: [[u8; 2]; 26],
    encrypted_dictionary: HashMap<Box<[u8]>, u8>,
}

impl Encrypter {
    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let mut encryption_map = [[u8::MAX; 2]; 26];

        keys.into_iter().zip(values).for_each(|(key, value)| {
            encryption_map[key as usize - usize::from(b'a')] = value.into_bytes().try_into().ok().unwrap();
        });

        let mut encrypted_dictionary = HashMap::with_capacity(dictionary.len());

        for word in dictionary {
            let encrypted_word = Self::encrypt_impl(&encryption_map, word);

            if !encrypted_word.is_empty() {
                match encrypted_dictionary.entry(encrypted_word.into_bytes().into_boxed_slice()) {
                    Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(1);
                    }
                }
            }
        }

        Self {
            encryption_map,
            encrypted_dictionary,
        }
    }

    fn encrypt_impl(encryption_map: &[[u8; 2]; 26], word: String) -> String {
        String::from_utf8(
            word.into_bytes()
                .into_iter()
                .flat_map(|c| encryption_map[usize::from(c) - usize::from(b'a')])
                .collect(),
        )
        .unwrap_or_default()
    }

    fn encrypt(&self, word1: String) -> String {
        Self::encrypt_impl(&self.encryption_map, word1)
    }

    fn decrypt(&self, word2: String) -> i32 {
        self.encrypted_dictionary
            .get(word2.as_bytes())
            .copied()
            .unwrap_or(0)
            .into()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Encrypter for Encrypter {
    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        Self::new(keys, values, dictionary)
    }

    fn encrypt(&self, word1: String) -> String {
        self.encrypt(word1)
    }

    fn decrypt(&self, word2: String) -> i32 {
        self.decrypt(word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Encrypter>();
    }
}
