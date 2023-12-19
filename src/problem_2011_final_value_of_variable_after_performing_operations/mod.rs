pub mod iterative;

pub trait Solution {
    fn final_value_after_operations(operations: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["--X", "X++", "X++"] as &[_], 1),
            (&["++X", "++X", "X++"], 3),
            (&["X++", "++X", "--X", "X--"], 0),
        ];

        for (operations, expected) in test_cases {
            assert_eq!(
                S::final_value_after_operations(operations.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
