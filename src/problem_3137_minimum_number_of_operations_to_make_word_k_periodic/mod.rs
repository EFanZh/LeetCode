pub mod hash_map;

pub trait Solution {
    fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("leetcodeleet", 4), 1), (("leetcoleet", 2), 3)];

        for ((word, k), expected) in test_cases {
            assert_eq!(S::minimum_operations_to_make_k_periodic(word.to_string(), k), expected);
        }
    }
}
