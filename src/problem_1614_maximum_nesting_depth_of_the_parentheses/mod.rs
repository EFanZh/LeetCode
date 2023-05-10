pub mod stack;

pub trait Solution {
    fn max_depth(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("(1+(2*3)+((8)/4))+1", 3), ("(1)+((2))+(((3)))", 3)];

        for (s, expected) in test_cases {
            assert_eq!(S::max_depth(s.to_string()), expected);
        }
    }
}
