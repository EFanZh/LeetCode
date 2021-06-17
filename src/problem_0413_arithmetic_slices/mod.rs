pub mod clever_iterative;
pub mod iterative;

pub trait Solution {
    fn number_of_arithmetic_slices(a: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 4] as &[_], 3), (&[1], 0), (&[1, 2, 3, 2, 1], 2)];

        for (a, expected) in test_cases {
            assert_eq!(S::number_of_arithmetic_slices(a.to_vec()), expected);
        }
    }
}
