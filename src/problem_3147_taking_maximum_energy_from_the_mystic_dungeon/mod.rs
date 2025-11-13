pub mod suffix_sums;

pub trait Solution {
    fn maximum_energy(energy: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[5, 2, -10, -5, 1] as &[_], 3), 3), ((&[-2, -3, -1], 2), -1)];

        for ((energy, k), expected) in test_cases {
            assert_eq!(S::maximum_energy(energy.to_vec(), k), expected);
        }
    }
}
