pub mod counting_sort;

pub trait Solution {
    fn transform_array(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4, 3, 2, 1] as &[_], &[0, 0, 1, 1] as &[_]),
            (&[1, 5, 1, 4, 2], &[0, 0, 1, 1, 1]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::transform_array(nums.to_vec()), expected);
        }
    }
}
