pub mod mathematical;

pub trait Solution {
    fn min_cost(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(3, 3), (4, 6)];

        for (n, expected) in test_cases {
            assert_eq!(S::min_cost(n), expected);
        }
    }
}
