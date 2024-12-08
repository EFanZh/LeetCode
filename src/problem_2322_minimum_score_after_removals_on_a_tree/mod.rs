pub mod dfs;

pub trait Solution {
    fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 5, 5, 4, 11] as &[_], &[[0, 1], [1, 2], [1, 3], [3, 4]] as &[_]),
                9,
            ),
            ((&[5, 5, 2, 4, 4, 2], &[[0, 1], [1, 2], [5, 2], [4, 3], [1, 3]]), 0),
            ((&[29, 29, 23, 32, 17], &[[3, 1], [2, 3], [4, 1], [0, 4]]), 15),
        ];

        for ((nums, edges), expected) in test_cases {
            assert_eq!(
                S::minimum_score(nums.to_vec(), edges.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
