pub mod sliding_window;

pub trait Solution {
    fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("011010", 2, 3), true), (("01101110", 2, 3), false)];

        for ((s, min_jump, max_jump), expected) in test_cases {
            assert_eq!(S::can_reach(s.to_string(), min_jump, max_jump), expected);
        }
    }
}
