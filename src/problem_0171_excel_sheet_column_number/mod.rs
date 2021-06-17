pub mod iterative;

pub trait Solution {
    fn title_to_number(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("A", 1), ("AB", 28), ("ZY", 701)];

        for (s, expected) in test_cases {
            assert_eq!(S::title_to_number(s.to_string()), expected);
        }
    }
}
