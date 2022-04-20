pub mod iterative;

pub trait Solution {
    fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1] as &[_], true),
            (&[0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1], false),
            (&[3, 3, 6, 5, -2, 2, 5, 1, -9, 4], true),
            (&[18, 12, -18, 18, -19, -1, 10, 10], true),
            (&[1, -1, 1, -1], false),
            (&[6, 1, 1, 13, -1, 0, -10, 20], false),
            (&[1, 1, 1, 1], false),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::can_three_parts_equal_sum(arr.to_vec()), expected);
        }
    }
}
