pub mod greedy;

pub trait Solution {
    fn min_set_size(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 3, 3, 3, 5, 5, 5, 2, 2, 7] as &[_], 2),
            (&[7, 7, 7, 7, 7, 7], 1),
            (&[1000, 1000, 3, 7], 1),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::min_set_size(arr.to_vec()), expected);
        }
    }
}
