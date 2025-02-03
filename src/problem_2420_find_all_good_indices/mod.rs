pub mod iterative;

pub trait Solution {
    fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 1, 1, 1, 3, 4, 1] as &[_], 2), &[2, 3] as &[_]),
            ((&[2, 1, 1, 2], 2), &[]),
            ((&[440_043, 276_285, 336_957], 1), &[1]),
            (
                (&[253_747, 459_932, 263_592, 354_832, 60_715, 408_350, 959_296], 2),
                &[3],
            ),
            (
                (
                    &[
                        388_589, 17_165, 726_687, 401_298, 600_033, 537_254, 301_052, 151_069, 399_955,
                    ],
                    4,
                ),
                &[],
            ),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::good_indices(nums.to_vec(), k), expected);
        }
    }
}
