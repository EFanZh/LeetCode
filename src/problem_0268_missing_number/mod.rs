pub mod pigeonhole;
pub mod reduce_sum;
pub mod reduce_xor;

pub trait Solution {
    fn missing_number(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 0, 1] as &[_], 2), (&[9, 6, 4, 2, 3, 5, 7, 0, 1], 8)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::missing_number(nums.to_vec()), expected);
        }
    }
}
