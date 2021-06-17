pub mod iterative;

pub trait Solution {
    fn convert_to_title(n: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, "A"), (28, "AB"), (701, "ZY")];

        for (n, expected) in test_cases {
            assert_eq!(S::convert_to_title(n), expected);
        }
    }
}
