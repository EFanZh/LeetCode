pub mod iterative;

pub trait Solution {
    fn max_product(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 4, 5, 2] as &[_], 12), (&[1, 5, 4, 5], 16), (&[3, 7], 12)];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_product(nums.to_vec()), expected);
        }
    }
}
