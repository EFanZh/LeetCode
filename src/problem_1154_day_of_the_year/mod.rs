pub mod pre_computed_sums;

pub trait Solution {
    fn day_of_year(date: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("2019-01-09", 9),
            ("2019-02-10", 41),
            ("2004-03-01", 61),
            ("2000-12-04", 339),
        ];

        for (date, expected) in test_cases {
            assert_eq!(S::day_of_year(date.to_string()), expected);
        }
    }
}
