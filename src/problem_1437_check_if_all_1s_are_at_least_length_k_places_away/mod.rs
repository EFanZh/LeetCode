pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn k_length_apart(nums: Vec<i32>, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 0, 0, 0, 1, 0, 0, 1] as &[_], 2), true),
            ((&[1, 0, 0, 1, 0, 1], 2), false),
            ((&[0, 0, 0], 2), true),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::k_length_apart(nums.to_vec(), k), expected);
        }
    }
}
