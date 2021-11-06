pub mod iterative;

pub trait Solution {
    fn find_replace_string(s: String, indices: Vec<i32>, sources: Vec<String>, targets: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                ("abcd", &[0, 2] as &[_], &["a", "cd"] as &[_], &["eee", "ffff"] as &[_]),
                "eeebffff",
            ),
            (("abcd", &[0, 2], &["ab", "ec"], &["eee", "ffff"]), "eeecd"),
        ];

        for ((s, indices, sources, targets), expected) in test_cases {
            assert_eq!(
                S::find_replace_string(
                    s.to_string(),
                    indices.to_vec(),
                    sources.iter().copied().map(str::to_string).collect(),
                    targets.iter().copied().map(str::to_string).collect()
                ),
                expected
            );
        }
    }
}
