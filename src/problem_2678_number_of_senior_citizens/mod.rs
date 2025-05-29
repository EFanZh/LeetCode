pub mod iterative;

pub trait Solution {
    fn count_seniors(details: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["7868190130M7522", "5303914400F9211", "9273338290F4010"] as &[_], 2),
            (&["1313579440F2036", "2921522980M5644"], 0),
        ];

        for (details, expected) in test_cases {
            assert_eq!(
                S::count_seniors(details.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
