pub mod brute_force;

pub trait Solution {
    fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[1, 2, 3] as &[_], &[4, 5, 6], &[7, 8, 9]] as &[&[_]],
                &[&[1, 4, 7] as &[_], &[2, 5, 8], &[3, 6, 9]] as &[&[_]],
            ),
            (&[&[1, 2, 3], &[4, 5, 6]], &[&[1, 4], &[2, 5], &[3, 6]]),
        ];

        for (matrix, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::transpose(matrix.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
