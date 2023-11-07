pub mod binary_heap;

pub trait Solution {
    fn minimum_deviation(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4] as &[_], 1),
            (&[4, 1, 5, 20, 3], 3),
            (&[2, 10, 8], 3),
            (&[4, 9, 4, 5], 5),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_deviation(nums.to_vec()), expected);
        }
    }
}
