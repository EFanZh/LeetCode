pub mod sliding_window;

pub trait Solution {
    fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[10, 5, 2, 6] as &[_], 100), 8), ((&[1, 2, 3], 0), 0)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::num_subarray_product_less_than_k(nums.to_vec(), k), expected);
        }
    }
}
