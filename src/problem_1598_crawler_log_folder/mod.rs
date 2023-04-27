pub mod stack;

pub trait Solution {
    fn min_operations(logs: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["d1/", "d2/", "../", "d21/", "./"] as &[_], 2),
            (&["d1/", "d2/", "./", "d3/", "../", "d31/"], 3),
            (&["d1/", "../", "../", "../"], 0),
        ];

        for (logs, expected) in test_cases {
            assert_eq!(
                S::min_operations(logs.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
