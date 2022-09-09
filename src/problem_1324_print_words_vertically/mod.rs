pub mod iterative;

pub trait Solution {
    fn print_vertically(s: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("HOW ARE YOU", &["HAY", "ORO", "WEU"] as &[_]),
            ("TO BE OR NOT TO BE", &["TBONTB", "OEROOE", "   T"]),
            ("CONTEST IS COMING", &["CIC", "OSO", "N M", "T I", "E N", "S G", "T"]),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::print_vertically(s.to_string()), expected);
        }
    }
}
