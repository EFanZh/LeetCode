pub mod bfs;

pub trait Solution {
    fn num_similar_groups(strs: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["tars", "rats", "arts", "star"] as &[_], 2),
            (&["omv", "ovm"], 1),
            (&["abc", "abc"], 1),
        ];

        for (strs, expected) in test_cases {
            assert_eq!(
                S::num_similar_groups(strs.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
