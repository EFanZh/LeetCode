pub mod pigeonhole;

pub trait Solution {
    fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 5, 1] as &[_], true),
            (&[1, 2, 4], false),
            (&[20, 11, 17, -2], false),
            (&[0, 0, 1], false),
            (&[1, 2, 3, 2, 5], false),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::can_make_arithmetic_progression(arr.to_vec()), expected);
        }
    }
}
