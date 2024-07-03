pub mod iterative;

pub trait Solution {
    fn divisor_substrings(num: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((240, 2), 2), ((430_043, 2), 2)];

        for ((num, k), expected) in test_cases {
            assert_eq!(S::divisor_substrings(num, k), expected);
        }
    }
}
