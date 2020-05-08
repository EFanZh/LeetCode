pub mod divide_and_conquer;
pub mod dynamic_programming;

pub trait Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A000170.
        let test_cases = [(&[-2, 1, -3, 4, -1, 2, 1, -5, 4] as &[_], 6)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::max_sub_array(nums.to_vec()), expected);
        }
    }
}
