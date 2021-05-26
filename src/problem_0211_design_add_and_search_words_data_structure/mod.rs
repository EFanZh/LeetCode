pub mod trie;

pub trait WordDictionary {
    fn new() -> Self;
    fn add_word(&mut self, word: String);
    fn search(&self, word: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::WordDictionary;

    enum Operation<'a> {
        AddWord(&'a str),
        Search(&'a str, bool),
    }

    pub fn run<D: WordDictionary>() {
        use Operation::{AddWord, Search};

        let test_cases = [
            &[
                AddWord("bad"),
                AddWord("dad"),
                AddWord("mad"),
                Search("pad", false),
                Search("bad", true),
                Search(".ad", true),
                Search("b..", true),
            ] as &[_],
            &[
                AddWord("a"),
                AddWord("a"),
                Search(".", true),
                Search("a", true),
                Search("aa", false),
                Search("a", true),
                Search(".a", false),
                Search("a.", false),
            ],
        ];

        for operations in test_cases.iter().copied() {
            let mut word_dictionary = D::new();

            for operation in operations {
                match *operation {
                    AddWord(word) => word_dictionary.add_word(word.to_string()),
                    Search(word, expected) => assert_eq!(word_dictionary.search(word.to_string()), expected),
                }
            }
        }
    }
}
