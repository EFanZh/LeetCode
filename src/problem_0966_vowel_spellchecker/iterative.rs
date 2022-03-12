pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let as_is = wordlist.iter().map(String::as_str).collect::<HashSet<_>>();
        let mut capitalized = HashMap::with_capacity(wordlist.len());
        let mut vowels = HashMap::with_capacity(wordlist.len());

        for word in &wordlist {
            capitalized.entry(word.to_ascii_uppercase()).or_insert(word.as_str());

            vowels
                .entry(word.to_ascii_uppercase().replace(['E', 'I', 'O', 'U'], "A"))
                .or_insert(word.as_str());
        }

        let mut result = queries;

        for word in &mut result {
            if !as_is.contains(word.as_str()) {
                word.make_ascii_uppercase();

                if let Some(&original) = capitalized.get(word.as_str()) {
                    word.clear();
                    word.push_str(original);
                } else if let Some(&original) = vowels.get(word.replace(['E', 'I', 'O', 'U'], "A").as_str()) {
                    word.clear();
                    word.push_str(original);
                } else {
                    word.clear();
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        Self::spellchecker(wordlist, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
