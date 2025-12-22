pub mod mathematical;

pub trait Solution {
    fn does_alice_win(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("leetcode", true), ("bbcd", false)];

        for (s, expected) in test_cases {
            assert_eq!(S::does_alice_win(s.to_string()), expected);
        }
    }
}
