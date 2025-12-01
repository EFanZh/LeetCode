pub mod modular_arithmetic;

pub trait Solution {
    fn count_complete_day_pairs(hours: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[12, 12, 30, 24, 24] as &[_], 2), (&[72, 48, 24, 3], 3)];

        for (hours, expected) in test_cases {
            assert_eq!(S::count_complete_day_pairs(hours.to_vec()), expected);
        }
    }
}
