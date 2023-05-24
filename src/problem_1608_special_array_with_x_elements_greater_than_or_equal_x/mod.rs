pub mod binary_heap;

pub trait Solution {
    fn special_array(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 5] as &[_], 2),
            (&[0, 0], -1),
            (&[0, 4, 3, 0, 4], 3),
            (&[3, 6, 7, 7, 0], -1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::special_array(nums.to_vec()), expected);
        }
    }
}
