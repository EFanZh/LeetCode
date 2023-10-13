pub mod prefix_sum;

pub trait Solution {
    fn min_characters(a: String, b: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("aba", "caa"), 2), (("dabadd", "cda"), 3)];

        for ((a, b), expected) in test_cases {
            assert_eq!(S::min_characters(a.to_string(), b.to_string()), expected);
        }
    }
}
