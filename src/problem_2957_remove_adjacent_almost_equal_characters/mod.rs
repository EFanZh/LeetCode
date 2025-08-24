pub mod greedy;

pub trait Solution {
    fn remove_almost_equal_characters(word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aaaaa", 2), ("abddez", 2), ("zyxyxyz", 3)];

        for (word, expected) in test_cases {
            assert_eq!(S::remove_almost_equal_characters(word.to_string()), expected);
        }
    }
}
