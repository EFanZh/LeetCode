pub mod hash_set;

pub trait Solution {
    fn is_path_crossing(path: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("NES", false), ("NESWW", true)];

        for (path, expected) in test_cases {
            assert_eq!(S::is_path_crossing(path.to_string()), expected);
        }
    }
}
