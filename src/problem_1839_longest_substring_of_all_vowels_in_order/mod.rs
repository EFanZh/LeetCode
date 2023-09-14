pub mod state_machine;

pub trait Solution {
    fn longest_beautiful_substring(word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("aeiaaioaaaaeiiiiouuuooaauuaeiu", 13),
            ("aeeeiiiioooauuuaeiou", 5),
            ("a", 0),
            ("iuaoeieaeeaeeiouueae", 7),
            ("eauoiouieaaoueiuaieoeauoiaueoiaeoiuieuaoiaeouiaueo", 0),
            ("aeuaeiiouoeiaeiiiii", 6),
            ("aeio", 0),
            ("aeioe", 0),
            ("aeioua", 5),
        ];

        for (word, expected) in test_cases {
            assert_eq!(S::longest_beautiful_substring(word.to_string()), expected);
        }
    }
}
