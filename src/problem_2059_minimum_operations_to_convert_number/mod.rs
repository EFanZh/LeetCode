pub mod bfs;

pub trait Solution {
    fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 4, 12] as &[_], 2, 12), 2),
            ((&[3, 5, 7], 0, -4), 2),
            ((&[2, 8, 16], 0, 1), -1),
        ];

        for ((nums, start, goal), expected) in test_cases {
            assert_eq!(S::minimum_operations(nums.to_vec(), start, goal), expected);
        }
    }
}
