pub struct Solution;

use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(PartialEq)]
struct HashMultiSet<T: Hash + Eq> {
    data: HashMap<T, usize>,
}

impl<T: Hash + Eq> HashMultiSet<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity(capacity),
        }
    }

    fn insert(&mut self, value: T) {
        match self.data.entry(value) {
            Entry::Occupied(entry) => *entry.into_mut() += 1,
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }
    }

    fn remove<Q: Hash + Eq + ?Sized>(&mut self, value: &Q)
    where
        T: Borrow<Q>,
    {
        let count = self.data.get_mut(value).unwrap();

        if *count == 1 {
            self.data.remove(value);
        } else {
            *count -= 1;
        }
    }

    fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T: Eq + Hash> FromIterator<T> for HashMultiSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut result = Self { data: HashMap::new() };

        result.extend(iter);

        result
    }
}

impl<T: Eq + Hash> Extend<T> for HashMultiSet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.insert(item)
        }
    }
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let s = s.into_bytes();
        let word_length = words.first().map_or(0, String::len);
        let num_words = words.len();
        let mut result = Vec::new();

        if let Some(last_split) = s.len().checked_sub(word_length * num_words) {
            let words = words.iter().map(String::as_bytes).collect::<HashMultiSet<_>>();
            let mut words_cache = HashMultiSet::with_capacity(num_words);

            for (i, chunks) in (0..word_length.min(last_split + 1))
                .map(|start| s[start..].chunks_exact(word_length))
                .enumerate()
            {
                words_cache.extend(chunks.clone().take(num_words));

                if words_cache == words {
                    result.push(i as _);
                }

                let current_chunks = chunks.clone().skip(num_words);

                for (j, (previous, current)) in (i + word_length..).step_by(word_length).zip(chunks.zip(current_chunks))
                {
                    words_cache.remove(previous);
                    words_cache.insert(current);

                    if words_cache == words {
                        result.push(j as _);
                    }
                }

                words_cache.clear();
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        Self::find_substring(s, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
