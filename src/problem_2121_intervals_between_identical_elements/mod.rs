pub mod iterative;

pub trait Solution {
    fn get_distances(arr: Vec<i32>) -> Vec<i64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 1, 3, 1, 2, 3, 3] as &[_], &[4_i64, 2, 7, 2, 4, 4, 5] as &[_]),
            (&[10, 5, 10, 10], &[5, 0, 3, 4]),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::get_distances(arr.to_vec()), expected);
        }
    }
}
