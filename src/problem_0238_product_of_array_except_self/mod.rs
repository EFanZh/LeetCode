pub mod two_passes;

pub trait Solution {
    fn product_except_self(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 4] as &[_], [24, 12, 8, 6])];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::product_except_self(nums.to_vec()), expected);
        }
    }
}
