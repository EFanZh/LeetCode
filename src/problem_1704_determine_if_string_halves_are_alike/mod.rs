pub mod iterative;

pub trait Solution {
    fn halves_are_alike(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("book", true), ("textbook", false), ("AbCdEfGh", true)];

        for (s, expected) in test_cases {
            assert_eq!(S::halves_are_alike(s.to_string()), expected);
        }
    }
}
