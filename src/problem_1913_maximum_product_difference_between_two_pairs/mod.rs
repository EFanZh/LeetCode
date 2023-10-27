pub mod greedy;

pub trait Solution {
    fn max_product_difference(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[5, 6, 2, 7, 4] as &[_], 34),
            (&[4, 2, 5, 9, 7, 4, 8], 64),
            (&[1, 6, 7, 5, 2, 4, 10, 6, 4], 68),
            (&[7, 6, 4, 3, 9, 8, 2, 1, 10], 88),
            (&[3, 7, 10, 2, 7, 7], 64),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_product_difference(nums.to_vec()), expected);
        }
    }
}
