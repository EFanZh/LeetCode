pub mod binary_search;

pub trait Solution {
    fn ship_within_days(weights: Vec<i32>, days: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10] as &[_], 5), 15),
            ((&[3, 2, 2, 4, 1, 4], 3), 6),
            ((&[1, 2, 3, 1, 1], 4), 3),
        ];

        for ((weights, days), expected) in test_cases {
            assert_eq!(S::ship_within_days(weights.to_vec(), days), expected);
        }
    }
}
