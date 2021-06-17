pub mod dynamic_programming;

pub trait Solution {
    fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[&[9, 9, 4] as &[_], &[6, 6, 8], &[2, 1, 1]] as &[&[_]], 4),
            (&[&[3, 4, 5], &[3, 2, 6], &[2, 2, 1]], 4),
        ];

        for (matrix, expected) in test_cases {
            assert_eq!(
                S::longest_increasing_path(matrix.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
