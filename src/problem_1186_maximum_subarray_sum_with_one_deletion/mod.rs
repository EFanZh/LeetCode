pub mod iterative;

pub trait Solution {
    fn maximum_sum(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, -2, 0, 3] as &[_], 4),
            (&[1, -2, -2, 3], 3),
            (&[-1, -1, -1, -1], -1),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::maximum_sum(arr.to_vec()), expected);
        }
    }
}
