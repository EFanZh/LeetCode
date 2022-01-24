pub mod brute_force;

pub trait Solution {
    fn superpalindromes_in_range(left: String, right: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("4", "1000"), 4),
            (("1", "2"), 1),
            (("111", "231"), 1),
            (("1", "999999999999999999"), 70),
            (("151807040", "999999999999999999"), 51),
            (("1036775601", "999999999999999999"), 49),
            (("150208069040", "999999999999999999"), 44),
            (("9007199254740992", "999999999999999999"), 22),
        ];

        for ((left, right), expected) in test_cases {
            assert_eq!(
                S::superpalindromes_in_range(left.to_string(), right.to_string()),
                expected
            );
        }
    }
}
