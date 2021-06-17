pub mod dynamic_programming;

pub trait Solution {
    fn k_inverse_pairs(n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A008302.

        let test_cases = [
            ((3, 0), 1),
            ((3, 1), 2),
            ((3, 2), 2),
            ((3, 3), 1),
            ((10, 7), 4489),
            ((1000, 1000), 663_677_020),
        ];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::k_inverse_pairs(n, k), expected);
        }
    }
}
