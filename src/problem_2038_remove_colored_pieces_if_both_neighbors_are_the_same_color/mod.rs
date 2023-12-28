pub mod iterative;

pub trait Solution {
    fn winner_of_game(colors: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("AAABABB", true), ("AA", false), ("ABBBBBBBAAA", false)];

        for (colors, expected) in test_cases {
            assert_eq!(S::winner_of_game(colors.to_string()), expected);
        }
    }
}
