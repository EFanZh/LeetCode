pub mod iterative;

pub trait Solution {
    fn decimal_representation(n: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(537, &[500, 30, 7] as &[_]), (102, &[100, 2]), (6, &[6])];

        for (n, expected) in test_cases {
            assert_eq!(S::decimal_representation(n), expected);
        }
    }
}
