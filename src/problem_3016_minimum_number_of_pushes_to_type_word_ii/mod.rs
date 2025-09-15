pub mod greedy;

pub trait Solution {
    fn minimum_pushes(word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abcde", 5), ("xyzxyzxyzxyz", 12), ("aabbccddeeffgghhiiiiii", 24)];

        for (word, expected) in test_cases {
            assert_eq!(S::minimum_pushes(word.to_string()), expected);
        }
    }
}
