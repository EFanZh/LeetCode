pub mod iterative;

pub trait Solution {
    fn num_of_pairs(nums: Vec<String>, target: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["777", "7", "77", "77"] as &[_], "7777"), 4),
            ((&["123", "4", "12", "34"], "1234"), 2),
            ((&["1", "1", "1"], "11"), 6),
            ((&["74", "1", "67", "1", "74"], "174"), 4),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(
                S::num_of_pairs(nums.iter().copied().map(str::to_string).collect(), target.to_string()),
                expected,
            );
        }
    }
}
