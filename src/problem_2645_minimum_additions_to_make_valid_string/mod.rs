pub mod greedy;

pub trait Solution {
    fn add_minimum(word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("b", 2), ("aaa", 6), ("abc", 0)];

        for (word, expected) in test_cases {
            assert_eq!(S::add_minimum(word.to_string()), expected);
        }
    }
}
