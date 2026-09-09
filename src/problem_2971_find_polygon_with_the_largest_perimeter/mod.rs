pub mod greedy_binary_heap;

pub trait Solution {
    fn largest_perimeter(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[5, 5, 5] as &[_], 15),
            (&[1, 12, 1, 2, 5, 50, 3], 12),
            (&[5, 5, 50], -1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::largest_perimeter(nums.to_vec()), expected);
        }
    }
}
