pub mod tail_recursive;

pub trait Solution {
    fn candy(ratings: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 0, 2] as &[_], 5),
            (&[1, 2, 2], 4),
            (&[29, 51, 87, 87, 72, 12], 12),
            (&[1, 2, 3, 5, 4, 3, 2, 1, 4, 3, 2, 1, 3, 2, 1, 1, 2, 3, 4], 47),
        ];

        for (ratings, expected) in test_cases.iter().copied() {
            assert_eq!(S::candy(ratings.to_vec()), expected);
        }
    }
}
