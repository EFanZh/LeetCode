pub mod hash_map;

pub trait Solution {
    fn array_rank_transform(arr: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[40, 10, 20, 30] as &[_], &[4, 1, 2, 3] as &[_]),
            (&[100, 100, 100], &[1, 1, 1]),
            (&[37, 12, 28, 9, 100, 56, 80, 5, 12], &[5, 3, 4, 2, 8, 6, 7, 1, 3]),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::array_rank_transform(arr.to_vec()), expected);
        }
    }
}
