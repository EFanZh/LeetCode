pub mod simulation;

pub trait Solution {
    fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], &[4, 5, 3, 2, 1] as &[_]), true),
            ((&[1, 2, 3, 4, 5], &[4, 3, 5, 1, 2]), false),
        ];

        for ((pushed, popped), expected) in test_cases {
            assert_eq!(S::validate_stack_sequences(pushed.to_vec(), popped.to_vec()), expected);
        }
    }
}
