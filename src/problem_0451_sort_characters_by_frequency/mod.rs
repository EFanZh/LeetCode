pub mod sort_counts;

pub trait Solution {
    fn frequency_sort(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("tree", &["eert", "eetr"] as &[_]),
            ("cccaaa", &["aaaccc", "cccaaa"]),
            ("Aabb", &["bbAa", "bbaA"]),
        ];

        for (s, expected) in test_cases {
            assert!(expected.contains(&S::frequency_sort(s.to_string()).as_str()));
        }
    }
}
