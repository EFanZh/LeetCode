pub mod mathematical;

pub trait Solution {
    fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, 10), 60), ((1, 2), 10)];

        for ((main_tank, additional_tank), expected) in test_cases {
            assert_eq!(S::distance_traveled(main_tank, additional_tank), expected);
        }
    }
}
