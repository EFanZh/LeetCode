pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn max_balanced_shipments(weight: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 5, 1, 4, 3] as &[_], 2), (&[4, 4], 0)];

        for (weight, expected) in test_cases {
            assert_eq!(S::max_balanced_shipments(weight.to_vec()), expected);
        }
    }
}
