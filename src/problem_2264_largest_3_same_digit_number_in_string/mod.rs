pub mod iterative;

pub trait Solution {
    fn largest_good_integer(num: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("6777133339", "777"),
            ("2300019", "000"),
            ("42352338", ""),
            ("222", "222"),
        ];

        for (num, expected) in test_cases {
            assert_eq!(S::largest_good_integer(num.to_string()), expected);
        }
    }
}
