pub mod iterative;

pub trait Solution {
    fn count_characters(words: Vec<String>, chars: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["cat", "bt", "hat", "tree"] as &[_], "atach"), 6),
            ((&["hello", "world", "leetcode"], "welldonehoneyr"), 10),
        ];

        for ((words, chars), expected) in test_cases {
            assert_eq!(
                S::count_characters(words.iter().copied().map(str::to_string).collect(), chars.to_string()),
                expected
            );
        }
    }
}
