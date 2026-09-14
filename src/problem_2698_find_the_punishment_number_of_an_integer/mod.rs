pub mod precomputed;

pub trait Solution {
    fn punishment_number(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(10, 182), (37, 1478)];

        for (n, expected) in test_cases {
            assert_eq!(S::punishment_number(n), expected);
        }
    }
}
