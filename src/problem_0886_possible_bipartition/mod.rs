pub mod bfs;

pub trait Solution {
    fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[[1, 2], [1, 3], [2, 4]] as &dyn Matrix<_>), true),
            ((3, &[[1, 2], [1, 3], [2, 3]]), false),
            ((5, &[[1, 2], [2, 3], [3, 4], [4, 5], [1, 5]]), false),
            ((5, &[[1, 2], [1, 3], [1, 4], [1, 5]]), true),
        ];

        for ((n, dislikes), expected) in test_cases {
            assert_eq!(S::possible_bipartition(n, dislikes.to_vec()), expected);
        }
    }
}
