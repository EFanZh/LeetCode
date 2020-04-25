pub mod iterative;

pub trait Solution {
    fn count_and_say(n: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, "1"), (2, "11"), (3, "21"), (4, "1211"), (5, "111221")];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::count_and_say(n), expected);
        }
    }
}
