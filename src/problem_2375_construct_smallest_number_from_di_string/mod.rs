pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn smallest_number(pattern: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("IIIDIDDD", "123549876"), ("DDD", "4321"), ("DDDIII", "4321567")];

        for (pattern, expected) in test_cases {
            assert_eq!(S::smallest_number(pattern.to_string()), expected);
        }
    }
}
