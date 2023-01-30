pub mod iterative;

pub trait Solution {
    fn min_number_of_frogs(croak_of_frogs: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("croakcroak", 1),
            ("crcoakroak", 2),
            ("croakcrook", -1),
            ("croakroak", -1),
            ("aoocrrackk", -1),
            ("croakcroa", -1),
            ("k", -1),
        ];

        for (croak_of_frogs, expected) in test_cases {
            assert_eq!(S::min_number_of_frogs(croak_of_frogs.to_string()), expected);
        }
    }
}
