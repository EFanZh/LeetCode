pub mod dynamic_programming;

pub trait Solution {
    fn maximum_product(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3] as &[_], 6), (&[1, 2, 3, 4], 24), (&[-1, -2, -3], -6)];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_product(nums.to_vec()), expected);
        }
    }
}
