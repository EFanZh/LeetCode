pub mod buckets;

pub trait Solution {
    fn similar_pairs(words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["aba", "aabb", "abcd", "bac", "aabc"] as &[_], 2),
            (&["aabb", "ab", "ba"], 3),
            (&["nba", "cba", "dba"], 0),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::similar_pairs(words.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
