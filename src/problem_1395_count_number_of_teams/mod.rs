pub mod merge_sort;

pub trait Solution {
    fn num_teams(rating: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 5, 3, 4, 1] as &[_], 3),
            (&[2, 1, 3], 0),
            (&[1, 2, 3, 4], 4),
            (&[4, 7, 9, 5, 10, 8, 2, 1, 6], 24),
        ];

        for (rating, expected) in test_cases {
            assert_eq!(S::num_teams(rating.to_vec()), expected);
        }
    }
}
