pub mod two_passes;
pub mod two_passes_2;

pub trait Solution {
    fn set_zeroes(matrix: &mut Vec<Vec<i32>>);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[1, 1, 1] as &[_], &[1, 0, 1], &[1, 1, 1]] as &[&[_]],
                &[&[1, 0, 1] as &[_], &[0, 0, 0], &[1, 0, 1]] as &[&[_]],
            ),
            (
                &[&[0, 1, 2, 0], &[3, 4, 5, 2], &[1, 3, 1, 5]],
                &[&[0, 0, 0, 0], &[0, 4, 5, 0], &[0, 3, 1, 0]],
            ),
        ];

        for (matrix, expected) in test_cases.iter().copied() {
            let mut matrix = matrix.iter().map(|row| row.to_vec()).collect();

            S::set_zeroes(&mut matrix);

            assert_eq!(matrix, expected);
        }
    }
}
