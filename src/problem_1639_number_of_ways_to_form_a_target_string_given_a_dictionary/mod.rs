pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn num_ways(words: Vec<String>, target: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["acca", "bbbb", "caca"] as &[_], "aba"), 6),
            ((&["abba", "baab"], "bab"), 4),
        ];

        for ((words, target), expected) in test_cases {
            assert_eq!(
                S::num_ways(words.iter().copied().map(str::to_string).collect(), target.to_string()),
                expected,
            );
        }
    }
}
