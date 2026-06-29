pub mod quick_select;

pub trait Solution {
    fn count_elements(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[3, 1, 2] as &[_], 1), 2), ((&[5, 5, 5], 2), 0)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::count_elements(nums.to_vec(), k), expected);
        }
    }
}
