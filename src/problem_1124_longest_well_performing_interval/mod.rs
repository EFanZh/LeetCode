pub mod monotonic_stack;

pub trait Solution {
    fn longest_wpi(hours: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[9, 9, 6, 0, 6, 6, 9] as &[_], 3), (&[6, 6, 6], 0), (&[6, 6, 9], 1)];

        for (hours, expected) in test_cases {
            assert_eq!(S::longest_wpi(hours.to_vec()), expected);
        }
    }
}
