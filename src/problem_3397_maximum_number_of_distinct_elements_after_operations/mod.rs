pub mod greedy;

pub trait Solution {
    fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2, 2, 3, 3, 4] as &[_], 2), 6), ((&[4, 4, 4, 4], 1), 3)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::max_distinct_elements(nums.to_vec(), k), expected);
        }
    }
}
