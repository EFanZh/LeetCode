pub mod modular_arithmetic;

pub trait Solution {
    fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, -10, 7, 13, 6, 8] as &[_], 5), 4),
            ((&[1, -10, 7, 13, 6, 8], 7), 2),
        ];

        for ((nums, value), expected) in test_cases {
            assert_eq!(S::find_smallest_integer(nums.to_vec(), value), expected);
        }
    }
}
