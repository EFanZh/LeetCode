pub mod binary_search;

pub trait Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[&[1, 3, 5, 7] as &[_], &[10, 11, 16, 20], &[23, 30, 34, 50]] as &[&[_]],
                    3,
                ),
                true,
            ),
            ((&[&[1, 3, 5, 7], &[10, 11, 16, 20], &[23, 30, 34, 50]], 13), false),
            ((&[&[]], 1), false),
            ((&[&[1, 3, 5, 7], &[10, 11, 16, 20], &[23, 30, 34, 50]], 11), true),
        ];

        for ((matrix, target), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::search_matrix(matrix.iter().map(|row| row.to_vec()).collect(), target),
                expected
            );
        }
    }
}
