pub mod mathematical;

pub trait Solution {
    fn kth_grammar(n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A010060.

        let test_cases = [((1, 1), 0), ((2, 1), 0), ((2, 2), 1), ((3, 1), 0)];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::kth_grammar(n, k), expected);
        }
    }
}
