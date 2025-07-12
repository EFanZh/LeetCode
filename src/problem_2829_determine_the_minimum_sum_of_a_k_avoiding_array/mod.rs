pub mod greedy;

pub trait Solution {
    fn minimum_sum(n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, 4), 18), ((2, 6), 3)];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::minimum_sum(n, k), expected);
        }
    }
}
