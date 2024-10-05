pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn maximum_and_sum(nums: Vec<i32>, num_slots: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5, 6] as &[_], 3), 9),
            ((&[1, 3, 10, 4, 7, 1], 9), 24),
            ((&[10, 5, 3, 6, 11, 8, 8], 4), 16),
            ((&[8, 13, 3, 15, 3, 15, 2, 15, 5, 7, 6], 8), 60),
        ];

        for ((nums, num_slots), expected) in test_cases {
            assert_eq!(S::maximum_and_sum(nums.to_vec(), num_slots), expected);
        }
    }
}
