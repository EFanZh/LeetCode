pub mod binary_search;
pub mod newtons_method;

pub trait Solution {
    fn is_perfect_square(num: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(16, true), (14, false), (2_147_395_600, true)];

        for (num, expected) in test_cases {
            assert_eq!(S::is_perfect_square(num), expected);
        }
    }
}
