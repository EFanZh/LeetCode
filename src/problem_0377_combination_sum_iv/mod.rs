pub mod dynamic_programming;
pub mod memoized_dynamic_programming;

pub trait Solution {
    fn combination_sum4(nums: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2, 3] as &[_], 4), 7), ((&[2, 1, 3], 35), 1_132_436_852)];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::combination_sum4(nums.to_vec(), target), expected);
        }
    }
}
