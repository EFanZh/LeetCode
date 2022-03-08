pub mod greedy;

pub trait Solution {
    fn min_deletion_size(strs: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["ca", "bb", "ac"] as &[_], 1),
            (&["xc", "yb", "za"], 0),
            (&["zyx", "wvu", "tsr"], 3),
            (&["xga", "xfb", "yfa"], 1),
        ];

        for (strs, expected) in test_cases {
            assert_eq!(
                S::min_deletion_size(strs.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
