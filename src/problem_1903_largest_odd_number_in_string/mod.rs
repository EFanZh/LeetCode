pub mod greedy;

pub trait Solution {
    fn largest_odd_number(num: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("52", "5"), ("4206", ""), ("35427", "35427")];

        for (num, expected) in test_cases {
            assert_eq!(S::largest_odd_number(num.to_string()), expected);
        }
    }
}
