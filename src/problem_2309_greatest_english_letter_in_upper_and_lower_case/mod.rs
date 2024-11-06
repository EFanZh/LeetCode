pub mod iterative;

pub trait Solution {
    fn greatest_letter(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("lEeTcOdE", "E"), ("arRAzFif", "R"), ("AbCdEfGhIjK", "")];

        for (s, expected) in test_cases {
            assert_eq!(S::greatest_letter(s.to_string()), expected);
        }
    }
}
