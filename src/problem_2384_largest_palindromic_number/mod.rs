pub mod greedy;

pub trait Solution {
    fn largest_palindromic(num: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("444947137", "7449447"), ("00009", "9"), ("6006", "6006")];

        for (num, expected) in test_cases {
            assert_eq!(S::largest_palindromic(num.to_string()), expected);
        }
    }
}
