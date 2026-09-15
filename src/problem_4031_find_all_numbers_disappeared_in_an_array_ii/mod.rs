pub mod iterative;

pub trait Solution {
    fn find_disappeared_numbers(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 9, 7] as &[_], 1, 12), &[[1, 2], [4, 6], [8, 8], [10, 12]] as &[_]),
            ((&[1, 1], 5, 7), &[[5, 7]]),
            ((&[2, 3, 5], 2, 3), &[]),
            (
                (&[426, 748, 98, 321, 438, 321, 682], 262, 384),
                &[[262, 320], [322, 384]],
            ),
        ];

        for ((nums, lower, upper), expected) in test_cases {
            assert_eq!(S::find_disappeared_numbers(nums.to_vec(), lower, upper), expected);
        }
    }
}
