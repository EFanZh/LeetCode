pub mod greedy_binary_heap;

pub trait Solution {
    fn halve_array(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5, 19, 8, 1] as &[_], 3), (&[3, 8, 20], 3)];

        for (nums, expected) in test_cases {
            assert_eq!(S::halve_array(nums.to_vec()), expected);
        }
    }
}
