pub mod greedy;

pub trait Solution {
    fn partition_array(nums: Vec<i32>, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4] as &[_], 2), true),
            ((&[3, 5, 2, 2], 2), true),
            ((&[1, 5, 2, 3], 3), false),
            (
                (
                    &[
                        35, 39, 65, 101, 101, 54, 1, 111, 8, 107, 96, 90, 91, 54, 115, 36, 46, 76, 111, 39, 29, 122, 4,
                        113, 101, 73, 125, 39, 124, 33, 82, 39,
                    ],
                    16,
                ),
                false,
            ),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::partition_array(nums.to_vec(), k), expected);
        }
    }
}
