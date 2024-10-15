pub mod iterative;

pub trait Solution {
    fn digit_count(num: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("1210", true), ("030", false)];

        for (num, expected) in test_cases {
            assert_eq!(S::digit_count(num.to_string()), expected);
        }
    }
}
