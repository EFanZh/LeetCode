pub mod iterative;

pub trait Solution {
    fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1] as &[_], 0), 0), ((&[0, 10], 2), 6), ((&[1, 3, 6], 3), 3)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::smallest_range_ii(nums.to_vec(), k), expected);
        }
    }
}
