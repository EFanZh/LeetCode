pub mod recursive;
pub mod recursive_2;

pub trait Solution {
    fn make_largest_special(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("11011000", "11100100"), ("10", "10"), ("101010", "101010")];

        for (s, expected) in test_cases {
            assert_eq!(S::make_largest_special(s.to_string()), expected);
        }
    }
}
