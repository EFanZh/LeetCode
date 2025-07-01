pub mod iterative;

pub trait Solution {
    fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[0, 1, 2, 3, 4] as &[_], 2), 3), ((&[5, 1, 4, 2, 2], 6), 0)];

        for ((hours, target), expected) in test_cases {
            assert_eq!(S::number_of_employees_who_met_target(hours.to_vec(), target), expected);
        }
    }
}
