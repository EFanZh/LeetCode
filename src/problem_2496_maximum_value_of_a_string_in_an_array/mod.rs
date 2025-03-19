pub mod iterative;

pub trait Solution {
    fn maximum_value(strs: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["alic3", "bob", "3", "4", "00000"] as &[_], 5),
            (&["1", "01", "001", "0001"], 1),
        ];

        for (strs, expected) in test_cases {
            assert_eq!(
                S::maximum_value(strs.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
