pub mod iterative;

pub trait Solution {
    fn generate_tag(caption: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("Leetcode daily streak achieved", "#leetcodeDailyStreakAchieved"),
            ("can I Go There", "#canIGoThere"),
            (
                "hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh",
                "#hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh",
            ),
        ];

        for (caption, expected) in test_cases {
            assert_eq!(S::generate_tag(caption.to_string()), expected);
        }
    }
}
