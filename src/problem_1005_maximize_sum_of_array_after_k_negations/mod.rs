pub mod greedy;

pub trait Solution {
    fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 2, 3] as &[_], 1), 5),
            ((&[3, -1, 0, 2], 3), 6),
            ((&[2, -3, -1, 5, -4], 2), 13),
            ((&[5, 6, 9, -3, 3], 2), 20),
            ((&[-4, -2, -3], 4), 5),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::largest_sum_after_k_negations(nums.to_vec(), k), expected);
        }
    }
}
