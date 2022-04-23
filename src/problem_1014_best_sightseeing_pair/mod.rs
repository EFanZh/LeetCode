pub mod iterative;

pub trait Solution {
    fn max_score_sightseeing_pair(values: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[8, 1, 5, 2, 6] as &[_], 11), (&[1, 2], 2), (&[2, 1], 2)];

        for (values, expected) in test_cases {
            assert_eq!(S::max_score_sightseeing_pair(values.to_vec()), expected);
        }
    }
}
