pub mod iterative;

pub trait Solution {
    fn find_score(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 1, 3, 4, 5, 2] as &[_], 7), (&[2, 3, 5, 1, 3, 2], 5)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_score(nums.to_vec()), expected);
        }
    }
}
