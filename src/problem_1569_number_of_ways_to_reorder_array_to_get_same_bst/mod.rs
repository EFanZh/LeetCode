pub mod recursive;

pub trait Solution {
    fn num_of_ways(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 1, 3] as &[_], 1), (&[3, 4, 5, 1, 2], 5), (&[1, 2, 3], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::num_of_ways(nums.to_vec()), expected);
        }
    }
}
