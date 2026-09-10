pub mod iterative;

pub trait Solution {
    fn count_special_integers(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 2, 1] as &[_], 1), (&[3, 3, 1, 2, 2, 1], 2)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_special_integers(nums.to_vec()), expected);
        }
    }
}
