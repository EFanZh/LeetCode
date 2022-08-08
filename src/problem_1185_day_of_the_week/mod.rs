pub mod sakamoto_s_methods;

pub trait Solution {
    fn day_of_the_week(day: i32, month: i32, year: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((31, 8, 2019), "Saturday"),
            ((18, 7, 1999), "Sunday"),
            ((15, 8, 1993), "Sunday"),
        ];

        for ((day, month, year), expected) in test_cases {
            assert_eq!(S::day_of_the_week(day, month, year), expected);
        }
    }
}
