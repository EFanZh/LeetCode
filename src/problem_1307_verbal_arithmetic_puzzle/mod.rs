pub mod brute_force;
pub mod dfs;

pub trait Solution {
    fn is_solvable(words: Vec<String>, result: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["SEND", "MORE"] as &[_], "MONEY"), true),
            ((&["SIX", "SEVEN", "SEVEN"], "TWENTY"), true),
            ((&["LEET", "CODE"], "POINT"), false),
            ((&["I", "THINK", "IT", "BE", "THINE"], "INDEED"), true),
            ((&["AA", "BB"], "AA"), false),
            ((&["A", "B"], "A"), true),
            ((&["A", "B"], "B"), true),
            ((&["THIS", "IS", "TOO"], "FUNNY"), true),
            ((&["CBA", "CBA", "CBA", "CBA", "CBA"], "EDD"), false),
            ((&["WE", "ARE"], "IT"), false),
        ];

        for ((words, result), expected) in test_cases {
            assert_eq!(
                S::is_solvable(words.iter().copied().map(str::to_string).collect(), result.to_string()),
                expected
            );
        }
    }
}
