pub mod mathematical;

pub trait Solution {
    fn is_robot_bounded(instructions: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("GGLLGG", true),
            ("GG", false),
            ("GL", true),
            ("LLGRL", true),
            ("LRRRRLLLRL", true),
            ("RLLGLRRRRGGRRRGLLRRR", true),
        ];

        for (instructions, expected) in test_cases {
            assert_eq!(S::is_robot_bounded(instructions.to_string()), expected);
        }
    }
}
