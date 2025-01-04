pub mod iterative;

pub trait Solution {
    fn min_number_of_hours(initial_energy: i32, initial_experience: i32, energy: Vec<i32>, experience: Vec<i32>)
        -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((5, 3, &[1, 4, 3, 2] as &[_], &[2, 6, 3, 1] as &[_]), 8),
            ((2, 4, &[1], &[3]), 0),
        ];

        for ((initial_energy, initial_experience, energy, experience), expected) in test_cases {
            assert_eq!(
                S::min_number_of_hours(initial_energy, initial_experience, energy.to_vec(), experience.to_vec()),
                expected
            );
        }
    }
}
