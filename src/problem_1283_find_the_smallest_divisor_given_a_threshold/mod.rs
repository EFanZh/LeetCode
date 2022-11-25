pub mod binary_search;

pub trait Solution {
    fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2, 5, 9] as &[_], 6), 5), ((&[44, 22, 33, 11, 1], 5), 44)];

        for ((nums, threshold), expected) in test_cases {
            assert_eq!(S::smallest_divisor(nums.to_vec(), threshold), expected);
        }
    }
}
