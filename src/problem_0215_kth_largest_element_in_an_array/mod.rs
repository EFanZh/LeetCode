pub mod quick_select;
pub mod randomized_quick_select;

pub trait Solution {
    fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 2, 1, 5, 6, 4] as &[_], 2), 5),
            ((&[3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4),
            ((&[99, 99], 1), 99),
        ];

        for ((nums, k), expected) in test_cases.iter().copied() {
            assert_eq!(S::find_kth_largest(nums.to_vec(), k), expected);
        }
    }
}
