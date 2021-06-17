pub mod dynamic_programming;

pub trait Solution {
    fn number_of_arithmetic_slices(a: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 4, 6, 8, 10] as &[_], 7),
            (&[2, 2, 3, 4], 2),
            (&[0, 2_000_000_000, -294_967_296], 0),
            (&[1, 1, 1, 1, 1], 16),
        ];

        for (a, expected) in test_cases {
            assert_eq!(S::number_of_arithmetic_slices(a.to_vec()), expected);
        }
    }
}
