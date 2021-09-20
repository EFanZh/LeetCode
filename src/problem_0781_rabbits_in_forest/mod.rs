pub mod buckets;

pub trait Solution {
    fn num_rabbits(answers: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 1, 2] as &[_], 5), (&[10, 10, 10], 11)];

        for (answers, expected) in test_cases {
            assert_eq!(S::num_rabbits(answers.to_vec()), expected);
        }
    }
}
