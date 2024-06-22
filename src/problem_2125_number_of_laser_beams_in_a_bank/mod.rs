pub mod iterative;

pub trait Solution {
    fn number_of_beams(bank: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["011001", "000000", "010100", "001000"] as &[_], 8),
            (&["000", "111", "000"], 0),
        ];

        for (bank, expected) in test_cases {
            assert_eq!(
                S::number_of_beams(bank.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
