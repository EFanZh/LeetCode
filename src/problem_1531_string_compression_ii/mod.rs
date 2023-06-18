pub mod dynamic_programming;

pub trait Solution {
    fn get_length_of_optimal_compression(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("aaabcccd", 2), 4),
            (("aabbaa", 2), 2),
            (("aaaaaaaaaaa", 0), 3),
            (
                (
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    0,
                ),
                4,
            ),
        ];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::get_length_of_optimal_compression(s.to_string(), k), expected);
        }
    }
}
