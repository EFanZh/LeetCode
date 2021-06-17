pub mod iterative;

pub trait Solution {
    fn generate(num_rows: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            5,
            &[&[1] as &[_], &[1, 1], &[1, 2, 1], &[1, 3, 3, 1], &[1, 4, 6, 4, 1]] as &[&[_]],
        )];

        for (num_rows, expected) in test_cases {
            assert_eq!(S::generate(num_rows), expected);
        }
    }
}
