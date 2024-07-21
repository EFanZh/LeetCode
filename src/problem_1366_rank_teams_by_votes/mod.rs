pub mod iterative;

pub trait Solution {
    fn rank_teams(votes: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["ABC", "ACB", "ABC", "ACB", "ACB"] as &[_], "ACB"),
            (&["WXYZ", "XYZW"], "XWYZ"),
            (&["ZMNAGUEDSJYLBOPHRQICWFXTVK"], "ZMNAGUEDSJYLBOPHRQICWFXTVK"),
            (&["BCA", "CAB", "CBA", "ABC", "ACB", "BAC"], "ABC"),
        ];

        for (votes, expected) in test_cases {
            assert_eq!(
                S::rank_teams(votes.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
