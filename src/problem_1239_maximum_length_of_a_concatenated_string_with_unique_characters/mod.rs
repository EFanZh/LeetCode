pub mod dfs;

pub trait Solution {
    fn max_length(arr: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["un", "iq", "ue"] as &[_], 4),
            (&["cha", "r", "act", "ers"], 6),
            (&["abcdefghijklmnopqrstuvwxyz"], 26),
            (&["aa", "bb"], 0),
            (
                &[
                    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
                ],
                16,
            ),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(
                S::max_length(arr.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
