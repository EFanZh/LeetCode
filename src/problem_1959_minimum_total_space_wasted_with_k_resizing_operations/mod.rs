pub mod dynamic_programming;

pub trait Solution {
    fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[10, 20] as &[_], 0), 10),
            ((&[10, 20, 30], 1), 10),
            ((&[10, 20, 15, 30, 20], 2), 15),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::min_space_wasted_k_resizing(nums.to_vec(), k), expected);
        }
    }
}
