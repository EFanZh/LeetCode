pub mod prefix_xor;

pub trait Solution {
    fn wonderful_substrings(word: String) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aba", 4), ("aabb", 9), ("he", 2)];

        for (word, expected) in test_cases {
            assert_eq!(S::wonderful_substrings(word.to_string()), expected);
        }
    }
}
