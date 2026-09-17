pub mod greedy;

pub trait Solution {
    fn min_operations(nums: Vec<i32>, target: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3] as &[_], &[2, 1, 3] as &[_]), 2),
            ((&[4, 1, 4], &[5, 1, 4]), 1),
            ((&[7, 3, 7], &[5, 5, 9]), 2),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec(), target.to_vec()), expected);
        }
    }
}
