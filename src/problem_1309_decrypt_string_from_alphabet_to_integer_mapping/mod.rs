pub mod iterative;

pub trait Solution {
    fn freq_alphabets(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("10#11#12", "jkab"), ("1326#", "acz"), ("26#11#418#5", "zkdre")];

        for (s, expected) in test_cases {
            assert_eq!(S::freq_alphabets(s.to_string()), expected);
        }
    }
}
