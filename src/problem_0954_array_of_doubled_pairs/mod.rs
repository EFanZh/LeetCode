pub mod iterative;

pub trait Solution {
    fn can_reorder_doubled(arr: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 1, 3, 6] as &[_], false),
            (&[2, 1, 2, 6], false),
            (&[4, -2, 2, -4], true),
            (&[2, 4, 0, 0, 8, 1], true),
            (&[1, 2, 1, -8, 8, -4, 4, -4, 2, -2], true),
            (&[-6, -3], true),
            (&[-5, -3], false),
            (&[1, 2, 4, 0], false),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::can_reorder_doubled(arr.to_vec()), expected);
        }
    }
}
