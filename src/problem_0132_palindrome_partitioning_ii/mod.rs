pub mod dynamic_programming;

pub trait Solution {
    fn min_cut(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aab", 1)];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::min_cut(s.to_string()), expected);
        }
    }
}
