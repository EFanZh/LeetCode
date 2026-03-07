pub mod iterative;

pub trait Solution {
    fn answer_string(word: String, num_friends: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("dbca", 2), "dbc"), (("gggg", 4), "g"), (("gh", 1), "gh")];

        for ((word, num_friends), expected) in test_cases {
            assert_eq!(S::answer_string(word.to_string(), num_friends), expected);
        }
    }
}
