pub mod greedy;

pub trait Solution {
    fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (3, &[[1, 2], [2, 3], [3, 1]] as &[_]),
            (4, &[[1, 2], [3, 4]]),
            (4, &[[1, 2], [2, 3], [3, 4], [4, 1], [1, 3], [2, 4]]),
            (1, &[]),
        ];

        for (n, paths) in test_cases {
            let result = S::garden_no_adj(n, Matrix::to_vec(paths));

            assert!(result.iter().all(|plant| (1..5).contains(plant)));

            for &[x, y] in paths {
                assert_ne!(result[x as usize - 1], result[y as usize - 1]);
            }
        }
    }
}
