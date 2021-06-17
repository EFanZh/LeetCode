pub mod dynamic_programming;

pub trait Solution {
    fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["10", "0001", "111001", "1", "0"] as &[_], 5, 3), 4),
            ((&["10", "0", "1"], 1, 1), 2),
        ];

        for ((strs, m, n), expected) in test_cases {
            assert_eq!(
                S::find_max_form(strs.iter().copied().map(str::to_string).collect(), m, n),
                expected
            );
        }
    }
}
