pub mod prefix_sum;

pub trait Solution {
    fn subarray_sum(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 1, 1] as &[_], 2), 2), ((&[1, 2, 3], 3), 2)];

        for ((nums, k), expected) in test_cases.iter().copied() {
            assert_eq!(S::subarray_sum(nums.to_vec(), k), expected);
        }
    }
}
