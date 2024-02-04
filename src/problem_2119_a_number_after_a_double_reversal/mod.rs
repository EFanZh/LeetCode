pub mod mathematical;

pub trait Solution {
    fn is_same_after_reversals(num: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(526, true), (1800, false), (0, true)];
        for (num, expected) in test_cases {
            assert_eq!(S::is_same_after_reversals(num), expected);
        }
    }
}
