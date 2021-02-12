pub mod cheating;
pub mod iterative;

pub trait Solution {
    #[allow(clippy::wrong_self_convention)]
    fn to_lower_case(str: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("Hello", "hello"), ("here", "here"), ("LOVELY", "lovely")];

        for (str, expected) in test_cases.iter().copied() {
            assert_eq!(S::to_lower_case(str.to_string()), expected);
        }
    }
}
