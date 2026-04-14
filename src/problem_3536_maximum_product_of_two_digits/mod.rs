pub mod iterative;

pub trait Solution {
    fn max_product(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(31, 3), (22, 4), (124, 8)];

        for (n, expected) in test_cases {
            assert_eq!(S::max_product(n), expected);
        }
    }
}
