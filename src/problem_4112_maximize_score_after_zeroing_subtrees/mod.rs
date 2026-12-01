pub mod recursive;

pub trait Solution {
    fn max_score(parent: Vec<i32>, op: Vec<i32>, score: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[-1, 0, 0, 0, 1, 2, 3, 3] as &[_],
                    &[1, 1, 0, 0, 1, 1, 0, 1] as &[_],
                    &[5, 6, 1, 2, 3, 2, 3, -3] as &[_],
                ),
                16,
            ),
            ((&[-1, 0, 0], &[0, 0, 0], &[1, 2, 3]), 6),
            ((&[-1], &[1], &[-10]), -10),
        ];

        for ((parent, op, score), expected) in test_cases {
            assert_eq!(S::max_score(parent.to_vec(), op.to_vec(), score.to_vec()), expected);
        }
    }
}
