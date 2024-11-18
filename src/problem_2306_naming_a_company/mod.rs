pub mod group_by_prefix;

pub trait Solution {
    fn distinct_names(ideas: Vec<String>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["coffee", "donuts", "time", "toffee"] as &[_], 6),
            (&["lack", "back"], 0),
        ];

        for (ideas, expected) in test_cases {
            assert_eq!(
                S::distinct_names(ideas.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
