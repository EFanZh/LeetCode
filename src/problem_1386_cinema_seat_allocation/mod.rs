pub mod iterative;

pub trait Solution {
    fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, &[[1, 2], [1, 3], [1, 8], [2, 6], [3, 1], [3, 10]] as &[_]), 4),
            ((2, &[[2, 1], [1, 8], [2, 6]]), 2),
            ((4, &[[4, 3], [1, 4], [4, 6], [1, 7]]), 4),
        ];

        for ((n, reserved_seats), expected) in test_cases {
            assert_eq!(
                S::max_number_of_families(n, reserved_seats.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
