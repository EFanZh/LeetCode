pub mod mathematical;

pub trait Solution {
    fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("a1", "c3"), true), (("a1", "h3"), false)];

        for ((coordinate1, coordinate2), expected) in test_cases {
            assert_eq!(
                S::check_two_chessboards(coordinate1.to_string(), coordinate2.to_string()),
                expected,
            );
        }
    }
}
