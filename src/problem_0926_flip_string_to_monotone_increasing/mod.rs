pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn min_flips_mono_incr(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("00110", 1), ("010110", 2), ("00011000", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_flips_mono_incr(s.to_string()), expected);
        }
    }
}
