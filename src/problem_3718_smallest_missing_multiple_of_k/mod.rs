pub mod iterative;

pub trait Solution {
    fn missing_multiple(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[8, 2, 3, 4, 6] as &[_], 2), 10), ((&[1, 4, 7, 10, 15], 5), 5)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::missing_multiple(nums.to_vec(), k), expected);
        }
    }
}
