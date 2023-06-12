pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn get_max_len(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, -2, -3, 4] as &[_], 4),
            (&[0, 1, -2, -3, -4], 3),
            (&[-1, -2, -3, 0, 1], 2),
            (&[-1, 2], 1),
            (&[0, 0, 0, 0, 0], 0),
            (
                &[
                    5, -20, -20, -39, -5, 0, 0, 0, 36, -32, 0, -7, -10, -7, 21, 20, -12, -34, 26, 2,
                ],
                8,
            ),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::get_max_len(nums.to_vec()), expected);
        }
    }
}
