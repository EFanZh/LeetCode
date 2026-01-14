pub mod greedy;

pub trait Solution {
    fn find_maximum_score(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 1, 5] as &[_], 7), (&[4, 3, 1, 3, 2], 16)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_maximum_score(nums.to_vec()), expected);
        }
    }
}
