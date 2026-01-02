pub mod dynamic_programming;

pub trait Solution {
    fn count_of_pairs(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 2] as &[_], 4), (&[5, 5, 5, 5], 126)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_of_pairs(nums.to_vec()), expected);
        }
    }
}
