pub mod dynamic_programming;

pub trait Solution {
    fn delete_and_earn(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 4, 2] as &[_], 6),
            (&[2, 2, 3, 3, 3, 4], 9),
            (&[1, 6, 3, 3, 8, 4, 8, 10, 1, 3], 43),
            (
                &[
                    10, 8, 4, 2, 1, 3, 4, 8, 2, 9, 10, 4, 8, 5, 9, 1, 5, 1, 6, 8, 1, 1, 6, 7, 8, 9, 1, 7, 6, 8, 4, 5,
                    4, 1, 5, 9, 8, 6, 10, 6, 4, 3, 8, 4, 10, 8, 8, 10, 6, 4, 4, 4, 9, 6, 9, 10, 7, 1, 5, 3, 4, 4, 8, 1,
                    1, 2, 1, 4, 1, 1, 4, 9, 4, 7, 1, 5, 1, 10, 3, 5, 10, 3, 10, 2, 1, 10, 4, 1, 1, 4, 1, 2, 10, 9, 7,
                    10, 1, 2, 7, 5,
                ],
                338,
            ),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::delete_and_earn(nums.to_vec()), expected);
        }
    }
}
