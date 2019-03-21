pub mod bfs;
pub mod bfs_single_loop;
pub mod bfs_with_tricks;

pub trait Solution {
    fn jump(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    fn run_tests<S: super::Solution>() {
        let test_cases = vec![(vec![2, 3, 1, 1, 4], 2)];

        for (nums, expected) in test_cases {
            assert_eq!(S::jump(nums), expected);
        }
    }

    #[test]
    fn bfs() {
        run_tests::<super::bfs::Solution>();
    }

    #[test]
    fn bfs_single_loop() {
        run_tests::<super::bfs_single_loop::Solution>();
    }

    #[test]
    fn bfs_with_tricks() {
        run_tests::<super::bfs_with_tricks::Solution>();
    }
}
