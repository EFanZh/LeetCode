pub mod reversed_iterator;
pub mod stack;

pub trait Solution {
    fn backspace_compare(s: String, t: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("ab#c", "ad#c"), true),
            (("ab##", "c#d#"), true),
            (("a##c", "#a#c"), true),
            (("a#c", "b"), false),
        ];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::backspace_compare(s.to_string(), t.to_string()), expected);
        }
    }
}
