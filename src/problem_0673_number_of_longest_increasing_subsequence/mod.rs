pub mod dynamic_programming;

pub trait Solution {
    fn find_number_of_lis(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 3, 5, 4, 7] as &[_], 2),
            (&[2, 2, 2, 2, 2], 5),
            (&[1, 2, 4, 3, 5, 4, 7, 2], 3),
            (&[1, 2, 3, 1, 2, 3, 1, 2, 3], 10),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_number_of_lis(nums.to_vec()), expected);
        }
    }
}
