pub mod iterative;

pub trait Solution {
    fn minimum_flips(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(7, 0), (10, 4)];

        for (n, expected) in test_cases {
            assert_eq!(S::minimum_flips(n), expected);
        }
    }
}
