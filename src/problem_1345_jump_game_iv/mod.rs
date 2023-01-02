pub mod bfs;

pub trait Solution {
    fn min_jumps(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[100, -23, -23, 404, 100, 23, 23, 23, 3, 404] as &[_], 3),
            (&[7], 0),
            (&[7, 6, 9, 6, 9, 6, 9, 7], 1),
            (
                &[
                    11, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
                ],
                2,
            ),
            (&[6, 1, 9], 2),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::min_jumps(arr.to_vec()), expected);
        }
    }
}
