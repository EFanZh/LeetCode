pub mod hash_map;

pub trait Solution {
    fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 3, 1, 4, 1, 3, 2] as &[_], &[0, 3, 5] as &[_]),
                &[2, -1, 3] as &[_],
            ),
            ((&[1, 2, 3, 4], &[0, 1, 2, 3]), &[-1, -1, -1, -1]),
            ((&[2, 10, 20, 20, 20], &[1, 4, 2]), &[-1, 1, 1]),
        ];

        for ((nums, queries), expected) in test_cases {
            assert_eq!(S::solve_queries(nums.to_vec(), queries.to_vec()), expected);
        }
    }
}
