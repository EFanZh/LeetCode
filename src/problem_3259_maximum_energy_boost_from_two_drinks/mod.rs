pub mod dynamic_programming;

pub trait Solution {
    fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 1] as &[_], &[3, 1, 1] as &[_]), 5),
            ((&[4, 1, 1], &[1, 1, 3]), 7),
        ];

        for ((energy_drink_a, energy_drink_b), expected) in test_cases {
            assert_eq!(
                S::max_energy_boost(energy_drink_a.to_vec(), energy_drink_b.to_vec()),
                expected,
            );
        }
    }
}
