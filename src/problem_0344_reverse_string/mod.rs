pub mod cheating;
pub mod indices;
pub mod iterator;

pub trait Solution {
    fn reverse_string(s: &mut Vec<char>);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("hello", "olleh"), ("Hannah", "hannaH")];

        for (s, expected) in test_cases {
            let mut s = s.chars().collect();

            S::reverse_string(&mut s);

            assert_eq!(s.into_iter().collect::<String>(), expected);
        }
    }
}
