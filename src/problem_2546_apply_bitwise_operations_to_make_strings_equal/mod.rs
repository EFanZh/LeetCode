pub mod mathematical;

pub trait Solution {
    fn make_strings_equal(s: String, target: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("1010", "0110"), true), (("11", "00"), false)];

        for ((s, target), expected) in test_cases {
            assert_eq!(S::make_strings_equal(s.to_string(), target.to_string()), expected);
        }
    }
}
