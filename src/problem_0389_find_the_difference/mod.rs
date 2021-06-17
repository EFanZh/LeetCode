pub mod reduce_xor;

pub trait Solution {
    fn find_the_difference(s: String, t: String) -> char;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abcd", "abcde"), 'e'),
            (("", "y"), 'y'),
            (("a", "aa"), 'a'),
            (("ae", "aea"), 'a'),
        ];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::find_the_difference(s.to_string(), t.to_string()), expected);
        }
    }
}
