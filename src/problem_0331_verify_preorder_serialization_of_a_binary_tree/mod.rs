pub mod iterative;
pub mod iterative_2;
pub mod iterative_3;
pub mod recursive;

pub trait Solution {
    fn is_valid_serialization(preorder: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("9,3,4,#,#,1,#,#,2,#,6,#,#", true),
            ("1,#", false),
            ("9,#,#,1", false),
            ("#,7,6,9,#,#,#", false),
        ];

        for (preorder, expected) in test_cases.iter().copied() {
            assert_eq!(S::is_valid_serialization(preorder.to_string()), expected);
        }
    }
}
