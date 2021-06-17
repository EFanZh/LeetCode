pub mod dynamic_programming;
pub mod use_largest_rectangle_in_histogram;

pub trait Solution {
    fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[
                &['1', '0', '1', '0', '0'] as &[_],
                &['1', '0', '1', '1', '1'],
                &['1', '1', '1', '1', '1'],
                &['1', '0', '0', '1', '0'],
            ] as &[&[_]],
            6,
        )];

        for (matrix, expected) in test_cases {
            assert_eq!(
                S::maximal_rectangle(matrix.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
