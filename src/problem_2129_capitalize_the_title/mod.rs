pub mod iterative;

pub trait Solution {
    fn capitalize_title(title: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("capiTalIze tHe titLe", "Capitalize The Title"),
            ("First leTTeR of EACH Word", "First Letter of Each Word"),
            ("i lOve leetcode", "i Love Leetcode"),
            ("L hV", "l hv"),
            ("l CCK n k", "l Cck n k"),
        ];

        for (title, expected) in test_cases {
            assert_eq!(S::capitalize_title(title.to_string()), expected);
        }
    }
}
