pub mod dynamic_programming;

pub trait Solution {
    fn max_product(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, -2, 4] as &[_], 6), (&[-2, 0, -1], 0), (&[2, 3, -2, 4], 6)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::max_product(nums.to_vec()), expected);
        }
    }
}
