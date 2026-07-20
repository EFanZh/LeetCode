pub mod iterative;

pub trait Solution {
    fn is_adjacent_diff_at_most_two(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("132", true), ("129", false)];

        for (s, expected) in test_cases {
            assert_eq!(S::is_adjacent_diff_at_most_two(s.to_string()), expected);
        }
    }
}
