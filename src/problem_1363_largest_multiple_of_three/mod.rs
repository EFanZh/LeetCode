pub mod greedy;

pub trait Solution {
    fn largest_multiple_of_three(digits: Vec<i32>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[8, 1, 9] as &[_], "981"),
            (&[8, 6, 7, 1, 0], "8760"),
            (&[1], ""),
            (&[0, 0, 0, 0, 0, 0], "0"),
            (&[9, 8, 6, 8, 6], "966"),
            (&[0, 0, 0, 0, 0, 1], "0"),
            (&[9, 7, 6, 7, 6], "966"),
            (&[1, 1, 1, 2], "111"),
            (&[5, 8], ""),
            (&[7, 1, 2, 4, 0, 0, 4, 0, 3, 8], "874431000"),
        ];

        for (digits, expected) in test_cases {
            assert_eq!(S::largest_multiple_of_three(digits.to_vec()), expected);
        }
    }
}
