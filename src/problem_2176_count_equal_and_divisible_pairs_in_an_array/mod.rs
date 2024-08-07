pub mod iterative;

pub trait Solution {
    fn count_pairs(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 1, 2, 2, 2, 1, 3] as &[_], 2), 4),
            ((&[1, 2, 3, 4], 1), 0),
            (
                (
                    &[
                        29, 91, 29, 29, 71, 81, 100, 100, 34, 29, 71, 71, 91, 78, 29, 34, 78, 91, 34, 29, 91, 91, 37,
                        71, 81, 71, 37, 37, 81, 71, 34, 78, 34, 29, 81, 29, 100, 81, 34, 34, 71, 29, 91, 34, 71, 29,
                        34, 71, 81, 91, 29, 100, 78, 91, 78, 81, 34, 71, 100, 71, 34, 100, 37, 29, 34, 34, 34, 78, 81,
                        29, 37, 100, 100, 29, 78, 34, 29, 81, 29, 81, 29, 37, 71, 81, 29, 37, 91, 29, 34, 34, 78, 100,
                        34, 78, 29, 34, 100, 81, 100, 100,
                    ],
                    10,
                ),
                200,
            ),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::count_pairs(nums.to_vec(), k), expected);
        }
    }
}
