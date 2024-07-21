pub mod iterative;

pub trait Solution {
    fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[1, 2, 10], [2, 3, 20], [2, 5, 25]] as &[_], 5),
                &[10, 55, 45, 25, 25] as &[_],
            ),
            ((&[[1, 2, 10], [2, 2, 15]], 2), &[10, 25]),
        ];

        for ((bookings, n), expected) in test_cases {
            assert_eq!(
                S::corp_flight_bookings(bookings.iter().map(Vec::from).collect(), n),
                expected,
            );
        }
    }
}
