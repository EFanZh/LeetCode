pub mod bfs;
pub mod bfs_single_loop;
pub mod bfs_with_tricks;

pub trait Solution {
    fn jump(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 1, 1, 4] as &[_], 2)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::jump(nums.to_vec()), expected);
        }
    }
}
