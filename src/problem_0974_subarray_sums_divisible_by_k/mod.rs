pub mod iterative;

pub trait Solution {
    fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[4, 5, 0, -2, -3, 1] as &[_], 5), 7), ((&[5], 9), 0)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::subarrays_div_by_k(nums.to_vec(), k), expected);
        }
    }
}
