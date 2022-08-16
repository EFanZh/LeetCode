pub mod dynamic_programming;

pub trait Solution {
    fn min_score_triangulation(values: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3] as &[_], 6), (&[3, 7, 4, 5], 144), (&[1, 3, 1, 4, 1, 5], 13)];

        for (values, expected) in test_cases {
            assert_eq!(S::min_score_triangulation(values.to_vec()), expected);
        }
    }
}
