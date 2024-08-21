pub mod prefix_sum;

pub trait Solution {
    fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, -1, 2] as &[_], 3), 1),
            ((&[0, 0, 0], 1), 2),
            ((&[22, 4, -25, -20, -15, 15, -16, 7, 19, -10, 0, -13, -14], -33), 4),
            (
                (
                    &[
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30827, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0,
                    ],
                    0,
                ),
                33,
            ),
            (
                (
                    &[
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 0, 0, 0,
                        0, -99, 0,
                    ],
                    0,
                ),
                60,
            ),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::ways_to_partition(nums.to_vec(), k), expected);
        }
    }
}
