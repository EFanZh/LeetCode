pub mod brute_force;

pub trait Solution {
    fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[allow(clippy::manual_assert)]
    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[10, 20, 5] as &[_], &[70, 50, 30] as &[_], 2), 105.0),
            ((&[3, 1, 10, 10, 1], &[4, 8, 2, 2, 7], 3), 30.666_666_666_666_668),
            (
                (
                    &[32, 43, 66, 9, 94, 57, 25, 44, 99, 19],
                    &[187, 366, 117, 363, 121, 494, 348, 382, 385, 262],
                    4,
                ),
                1528.0,
            ),
            ((&[3, 4, 3], &[13, 8, 20], 1), 8.0),
        ];

        for ((quality, wage, k), expected) in test_cases {
            approx::assert_ulps_eq!(S::mincost_to_hire_workers(quality.to_vec(), wage.to_vec(), k), expected);
        }
    }
}
