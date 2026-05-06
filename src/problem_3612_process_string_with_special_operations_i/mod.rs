pub mod queue;

pub trait Solution {
    fn process_str(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("a#b%*", "ba"), ("z*#", ""), ("%*xilt**s#", "xisxis")];

        for (s, expected) in test_cases {
            assert_eq!(S::process_str(s.to_string()), expected);
        }
    }
}
