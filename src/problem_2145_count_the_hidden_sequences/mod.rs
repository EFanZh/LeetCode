pub mod iterative;

pub trait Solution {
    fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, -3, 4] as &[_], 1, 6), 2),
            ((&[3, -4, 5, 1, -2], -4, 5), 4),
            ((&[4, -7, 2], 3, 6), 0),
            ((&[-40], -46, 53), 60),
            (
                (&[16550, -34031, 14413, 17198, 9481, -3532, -27686, 2316], -6880, 39458),
                5247,
            ),
        ];

        for ((differences, lower, upper), expected) in test_cases {
            assert_eq!(S::number_of_arrays(differences.to_vec(), lower, upper), expected);
        }
    }
}
