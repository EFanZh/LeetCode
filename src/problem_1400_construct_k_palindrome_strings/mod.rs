pub mod mathematical;

pub trait Solution {
    fn can_construct(s: String, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("annabelle", 2), true), (("leetcode", 3), false), (("true", 4), true)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::can_construct(s.to_string(), k), expected);
        }
    }
}
