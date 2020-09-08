pub mod canonical;

pub trait Trie {
    fn new() -> Self;
    fn insert(&mut self, word: String);
    fn search(&self, word: String) -> bool;
    fn starts_with(&self, prefix: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Trie;

    enum Operation<'a> {
        Insert(&'a str),
        Search(&'a str, bool),
        StartsWith(&'a str, bool),
    }

    pub fn run<T: Trie>() {
        use Operation::{Insert, Search, StartsWith};

        let test_cases = [&[
            Insert("apple"),
            Search("apple", true),
            Search("app", false),
            StartsWith("app", true),
            Insert("app"),
            Search("app", true),
        ] as &[_]];

        for operations in test_cases.iter().copied() {
            let mut trie = T::new();

            for operation in operations {
                match *operation {
                    Insert(word) => trie.insert(word.to_string()),
                    Search(word, expected) => assert_eq!(trie.search(word.to_string()), expected),
                    StartsWith(prefix, expected) => assert_eq!(trie.starts_with(prefix.to_string()), expected),
                }
            }
        }
    }
}
