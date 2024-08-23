pub mod dfs;

pub trait Solution {
    fn maximum_invitations(favorite: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 2, 1, 2] as &[_], 3),
            (&[1, 2, 0], 3),
            (&[3, 0, 1, 4, 1], 4),
            (&[1, 0, 3, 2, 5, 6, 7, 4, 9, 8, 11, 10, 11, 12, 10], 11),
        ];

        for (favorite, expected) in test_cases {
            assert_eq!(S::maximum_invitations(favorite.to_vec()), expected);
        }
    }
}
