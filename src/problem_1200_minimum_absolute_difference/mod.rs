pub mod sort_and_check;

pub trait Solution {
    fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4, 2, 1, 3] as &[_], &[[1, 2], [2, 3], [3, 4]] as &[_]),
            (&[1, 3, 6, 10, 15], &[[1, 3]]),
            (&[3, 8, -10, 23, 19, -4, -14, 27], &[[-14, -10], [19, 23], [23, 27]]),
            (&[40, 11, 26, 27, -20], &[[26, 27]]),
            (
                &[
                    -169, -133, 178, -50, -4, 138, 136, -140, 137, 69, 8, -80, 3, 183, -57, 164, 192, 191,
                ],
                &[[136, 137], [137, 138], [191, 192]],
            ),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::minimum_abs_difference(arr.to_vec()), expected);
        }
    }
}
