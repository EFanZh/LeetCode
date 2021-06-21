pub mod greedy;

pub trait Solution {
    fn predict_party_victory(senate: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("RD", "Radiant"), ("RDD", "Dire"), ("DDRRR", "Dire")];

        for (senate, expected) in test_cases {
            assert_eq!(S::predict_party_victory(senate.to_string()), expected);
        }
    }
}
