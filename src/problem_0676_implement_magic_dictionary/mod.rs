pub mod hash_map;
pub mod hash_map_2;
pub mod trie;
pub mod trie_2;
pub mod trie_3;

pub trait MagicDictionary {
    fn new() -> Self;
    fn build_dict(&mut self, dictionary: Vec<String>);
    fn search(&self, search_word: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::MagicDictionary;

    pub fn run<D: MagicDictionary>() {
        let test_cases = [
            (
                &["hello", "leetcode"] as &[_],
                &[("hello", false), ("hhllo", true), ("hell", false), ("leetcoded", false)] as &[_],
            ),
            (
                &["hello", "hallo", "leetcode"],
                &[("hello", true), ("hhllo", true), ("hell", false), ("leetcoded", false)],
            ),
            (
                &["hello", "hallo", "leetcode"],
                &[("hello", true), ("hallo", true), ("hell", false), ("leetcodd", true)],
            ),
        ];

        for (dictionary, searches) in test_cases {
            let mut magic_dictionary = D::new();

            magic_dictionary.build_dict(dictionary.iter().copied().map(str::to_string).collect());

            for &(search_word, expected) in searches {
                assert_eq!(magic_dictionary.search(search_word.to_string()), expected);
            }
        }
    }
}
