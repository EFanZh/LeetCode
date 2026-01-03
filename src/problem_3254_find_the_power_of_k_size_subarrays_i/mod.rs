pub mod sliding_window;

pub trait Solution {
    fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 3, 2, 5] as &[_], 3), &[3, 4, -1, -1, -1] as &[_]),
            ((&[2, 2, 2, 2, 2], 4), &[-1, -1]),
            ((&[3, 2, 3, 2, 3, 2], 2), &[-1, 3, -1, 3, -1]),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::results_array(nums.to_vec(), k), expected);
        }
    }
}
