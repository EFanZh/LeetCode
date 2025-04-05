pub mod binary_heap;

pub trait Solution {
    fn max_kelements(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[10, 10, 10, 10, 10] as &[_], 5), 50), ((&[1, 10, 3, 3, 3], 3), 17)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::max_kelements(nums.to_vec(), k), expected);
        }
    }
}
