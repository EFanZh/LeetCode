pub mod iterative;

pub trait Solution {
    fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 1, 5] as &[_], &[2, 7, 4] as &[_]), 4),
            ((&[4, 1, 5, 9], &[1, 3, 2, 6]), 7),
            ((&[2, 2, 6, 6], &[1, 3, 2, 6]), 4),
        ];

        for ((seats, students), expected) in test_cases {
            assert_eq!(S::min_moves_to_seat(seats.to_vec(), students.to_vec()), expected);
        }
    }
}
