pub mod iterative;

pub trait Solution {
    fn count_tested_devices(battery_percentages: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 1, 2, 1, 3] as &[_], 3), (&[0, 1, 2], 2)];

        for (battery_percentages, expected) in test_cases {
            assert_eq!(S::count_tested_devices(battery_percentages.to_vec()), expected);
        }
    }
}
