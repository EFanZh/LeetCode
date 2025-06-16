pub mod greedy;

pub trait Solution {
    fn find_value_of_partition(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 2, 4] as &[_], 1), (&[100, 1, 10], 9)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_value_of_partition(nums.to_vec()), expected);
        }
    }
}
