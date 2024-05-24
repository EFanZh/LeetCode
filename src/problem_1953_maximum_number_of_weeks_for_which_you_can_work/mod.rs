pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn number_of_weeks(milestones: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3] as &[_], 6),
            (&[5, 2, 1], 7),
            (&[5, 9, 4, 4, 8, 9, 9, 8, 7, 3], 66),
            (&[5, 1, 5], 11),
            (&[i32::MAX, i32::MAX, i32::MAX], 6_442_450_941),
        ];

        for (milestones, expected) in test_cases {
            assert_eq!(S::number_of_weeks(milestones.to_vec()), expected);
        }
    }
}
