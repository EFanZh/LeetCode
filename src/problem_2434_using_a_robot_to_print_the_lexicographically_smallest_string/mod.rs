pub mod greedy;

pub trait Solution {
    fn robot_with_string(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("zza", "azz"),
            ("bac", "abc"),
            ("bdda", "addb"),
            ("vzhofnpo", "fnohopzv"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::robot_with_string(s.to_string()), expected);
        }
    }
}
