pub mod precompute;

pub trait TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self;
    fn q(&self, t: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::TopVotedCandidate;

    pub fn run<S: TopVotedCandidate>() {
        let test_cases = [
            (
                (&[0, 1, 1, 0, 0, 1, 0] as &[_], &[0, 5, 10, 15, 20, 25, 30] as &[_]),
                &[(3, 0), (12, 1), (25, 1), (15, 0), (24, 0), (8, 1)] as &[_],
            ),
            (
                (&[0, 0, 0, 0, 1], &[0, 6, 39, 52, 75]),
                &[
                    (45, 0),
                    (49, 0),
                    (59, 0),
                    (68, 0),
                    (42, 0),
                    (37, 0),
                    (99, 0),
                    (26, 0),
                    (78, 0),
                    (43, 0),
                ],
            ),
        ];

        for ((persons, times), qs) in test_cases {
            let top_voted_candidate = S::new(persons.to_vec(), times.to_vec());

            for &(t, expected) in qs {
                assert_eq!(top_voted_candidate.q(t), expected);
            }
        }
    }
}
