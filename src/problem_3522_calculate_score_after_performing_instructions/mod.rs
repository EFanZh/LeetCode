pub mod brute_force;

pub trait Solution {
    fn calculate_score(instructions: Vec<String>, values: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["jump", "add", "add", "jump", "add", "jump"] as &[_],
                    &[2, 1, 3, 1, -2, -3] as &[_],
                ),
                1,
            ),
            ((&["jump", "add", "add"], &[3, 1, 1]), 0),
            ((&["jump"], &[0]), 0),
        ];

        for ((instructions, values), expected) in test_cases {
            assert_eq!(
                S::calculate_score(
                    instructions.iter().copied().map(str::to_string).collect(),
                    values.to_vec()
                ),
                expected,
            );
        }
    }
}
