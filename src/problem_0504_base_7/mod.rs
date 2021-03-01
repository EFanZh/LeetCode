pub mod iterative;

pub trait Solution {
    fn convert_to_base7(num: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(100, "202"), (-7, "-10"), (0, "0")];

        for (num, expected) in test_cases.iter().copied() {
            assert_eq!(S::convert_to_base7(num), expected);
        }
    }
}
