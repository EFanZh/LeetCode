pub mod sliding_window;

pub trait Solution {
    fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[5, 3, 3, 3, 5, 6, 2] as &[_], 2), &[2, 3] as &[_]),
            ((&[1, 1, 1, 1, 1], 0), &[0, 1, 2, 3, 4]),
            ((&[1, 2, 3, 4, 5, 6], 2), &[]),
            ((&[1, 2, 5, 4, 1, 0, 2, 4, 5, 3, 1, 2, 4, 3, 2, 4, 8], 2), &[5, 10, 14]),
            ((&[1], 5), &[]),
        ];

        for ((security, time), expected) in test_cases {
            assert_eq!(S::good_days_to_rob_bank(security.to_vec(), time), expected);
        }
    }
}
