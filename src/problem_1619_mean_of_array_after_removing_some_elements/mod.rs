pub mod quick_select;

pub trait Solution {
    fn trim_mean(arr: Vec<i32>) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3] as &[_],
                2.0,
            ),
            (&[6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0], 4.0),
            (
                &[
                    6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5, 10, 8, 6, 6, 1,
                    0, 6, 10, 8, 2, 3, 4,
                ],
                43.0 / 9.0,
            ),
        ];

        for (arr, expected) in test_cases {
            approx::assert_ulps_eq!(S::trim_mean(arr.to_vec()), expected);
        }
    }
}
