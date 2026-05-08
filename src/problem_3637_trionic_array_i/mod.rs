pub mod iterative;

pub trait Solution {
    fn is_trionic(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 3, 5, 4, 2, 6] as &[_], true),
            (&[2, 1, 3], false),
            (&[1, 2, 3], false),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::is_trionic(nums.to_vec()), expected);
        }
    }
}
