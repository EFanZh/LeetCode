pub mod brute_force;

pub trait Solution {
    fn largest_integer(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 9, 2, 1, 7] as &[_], 3), 7),
            ((&[3, 9, 7, 2, 1, 7], 4), 3),
            ((&[0, 0], 1), -1),
            ((&[0, 0], 2), 0),
            ((&[], 1), -1),
            ((&[3, 1, 7, 10, 0], 1), 10),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::largest_integer(nums.to_vec(), k), expected);
        }
    }
}
