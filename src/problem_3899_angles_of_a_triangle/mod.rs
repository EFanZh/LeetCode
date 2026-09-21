pub mod mathematical;

pub trait Solution {
    fn internal_angles(sides: Vec<i32>) -> Vec<f64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[3, 4, 5],
                &[36.869_897_645_844_01, 53.130_102_354_155_99, 90.0] as &[_],
            ),
            (&[2, 4, 2], &[]),
        ];

        for (sides, expected) in test_cases {
            let result = S::internal_angles(sides.to_vec());

            assert_eq!(result.len(), expected.len());

            result
                .iter()
                .zip(expected)
                .for_each(|(&lhs, &rhs)| approx::assert_ulps_eq!(lhs, rhs));
        }
    }
}
