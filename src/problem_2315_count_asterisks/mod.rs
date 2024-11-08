pub mod iterative;

pub trait Solution {
    fn count_asterisks(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("l|*e*et|c**o|*de|", 2),
            ("iamprogrammer", 0),
            ("yo|uar|e**|b|e***au|tifu|l", 5),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::count_asterisks(s.to_string()), expected);
        }
    }
}
