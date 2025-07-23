pub mod combinations;

pub trait Solution {
    fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[5, 10, 1, 5, 2] as &[_], 1), 13), ((&[4, 3, 2, 1], 2), 1)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::sum_indices_with_k_set_bits(nums.to_vec(), k), expected);
        }
    }
}
