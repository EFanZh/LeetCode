pub mod iterative;

pub trait Solution {
    fn count_valid_words(sentence: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("cat and  dog", 3),
            ("!this  1-s b8d!", 0),
            ("alice and  bob are playing stone-game10", 5),
            ("!", 1),
            (".", 1),
            ("a-!b", 0),
            ("he bought 2 pencils, 3 erasers, and 1  pencil-sharpener.", 6),
            ("b-a-c f-d", 1),
        ];

        for (sentence, expected) in test_cases {
            assert_eq!(S::count_valid_words(sentence.to_string()), expected);
        }
    }
}
