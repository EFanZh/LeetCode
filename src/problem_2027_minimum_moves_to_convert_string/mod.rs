pub mod iterative;

pub trait Solution {
    fn minimum_moves(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("XXX", 1), ("XXOX", 2), ("OOOO", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::minimum_moves(s.to_string()), expected);
        }
    }
}
