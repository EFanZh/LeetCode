pub mod mathematical;

pub trait Solution {
    fn value_after_k_seconds(n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((4, 5), 56), ((5, 3), 35)];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::value_after_k_seconds(n, k), expected);
        }
    }
}
