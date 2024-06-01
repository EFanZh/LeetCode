pub mod sliding_window;

pub trait Solution {
    fn count_vowel_substrings(word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("aeiouu", 2),
            ("unicornarihan", 0),
            ("cuaieuouac", 7),
            ("wseyzauiagenl", 0),
            ("vnxaclbfiwdagzghmho", 0),
            ("poazaeuioauoiioaouuouaui", 31),
            ("ragvjaoeaieauuaaeaiii", 18),
        ];

        for (word, expected) in test_cases {
            assert_eq!(S::count_vowel_substrings(word.to_string()), expected);
        }
    }
}
