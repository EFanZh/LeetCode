pub mod greedy;

pub trait Solution {
    fn largest_integer(n: i32, s: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 9), 90), ((2, 19), -1), ((5, 0), 0)];

        for ((n, s), expected) in test_cases {
            assert_eq!(S::largest_integer(n, s), expected);
        }
    }
}
