pub mod hash_set;
pub mod two_runners;

pub trait Solution {
    fn is_happy(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(19, true)];

        for (n, expected) in test_cases {
            assert_eq!(S::is_happy(n), expected);
        }
    }
}
