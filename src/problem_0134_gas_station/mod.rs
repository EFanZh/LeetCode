pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], &[3, 4, 5, 1, 2] as &[_]), 3),
            ((&[2, 3, 4], &[3, 4, 3]), -1),
        ];

        for ((gas, cost), expected) in test_cases.iter().copied() {
            assert_eq!(S::can_complete_circuit(gas.to_vec(), cost.to_vec()), expected);
        }
    }
}
