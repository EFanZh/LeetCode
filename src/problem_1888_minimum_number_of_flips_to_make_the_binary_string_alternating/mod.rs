pub mod sliding_window;

pub trait Solution {
    fn min_flips(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("111000", 2), ("010", 0), ("1110", 1), ("10100101011001111110", 10)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_flips(s.to_string()), expected);
        }
    }
}
