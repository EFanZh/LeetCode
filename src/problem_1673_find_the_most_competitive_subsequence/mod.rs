pub mod montonic_stack;

pub trait Solution {
    fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 5, 2, 6] as &[_], 2), &[2, 6] as &[_]),
            ((&[2, 4, 3, 3, 5, 4, 9, 6], 4), &[2, 3, 3, 4]),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::most_competitive(nums.to_vec(), k), expected);
        }
    }
}
