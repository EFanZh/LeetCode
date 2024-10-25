pub mod iterative;

pub trait Solution {
    fn partition_array(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 6, 1, 2, 5] as &[_], 2), 2),
            ((&[1, 2, 3], 1), 2),
            ((&[2, 2, 4, 5], 0), 3),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::partition_array(nums.to_vec(), k), expected);
        }
    }
}
