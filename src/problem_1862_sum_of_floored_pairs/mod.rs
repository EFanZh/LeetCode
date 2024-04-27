pub mod iterative;

pub trait Solution {
    fn sum_of_floored_pairs(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 5, 9] as &[_], 10),
            (&[7, 7, 7, 7, 7, 7, 7], 49),
            (&[4, 3, 4, 3, 5], 17),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::sum_of_floored_pairs(nums.to_vec()), expected);
        }
    }
}
