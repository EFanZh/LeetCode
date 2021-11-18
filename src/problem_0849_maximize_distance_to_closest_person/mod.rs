pub mod iterative;

pub trait Solution {
    fn max_dist_to_closest(seats: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 0, 0, 0, 1, 0, 1] as &[_], 2),
            (&[1, 0, 0, 0], 3),
            (&[0, 1], 1),
            (&[1, 1, 0, 1, 1], 1),
        ];

        for (seats, expected) in test_cases {
            assert_eq!(S::max_dist_to_closest(seats.to_vec()), expected);
        }
    }
}
