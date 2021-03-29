pub mod memoized_dynamic_programming;

pub trait Solution {
    fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((10, 11), false),
            ((10, 0), true),
            ((10, 1), true),
            ((18, 79), true),
            ((5, 50), false),
            ((7, 23), true),
            ((8, 36), false),
        ];

        for ((max_choosable_integer, desired_total), expected) in test_cases.iter().copied() {
            assert_eq!(S::can_i_win(max_choosable_integer, desired_total), expected);
        }
    }
}
