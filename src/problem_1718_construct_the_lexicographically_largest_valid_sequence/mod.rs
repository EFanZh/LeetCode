pub mod iterative;

pub trait Solution {
    fn construct_distanced_sequence(n: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(3, &[3, 1, 2, 3, 2] as &[_]), (5, &[5, 3, 1, 4, 3, 5, 2, 4, 2])];

        for (n, expected) in test_cases {
            assert_eq!(S::construct_distanced_sequence(n), expected);
        }
    }
}
