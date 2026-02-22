pub mod dynamic_programming;

pub trait Solution {
    fn find_minimum_time(strength: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 4, 1] as &[_], 1), 4),
            ((&[2, 5, 4], 2), 5),
            ((&[25, 12, 21, 40, 20, 41, 14, 42], 4), 28),
        ];

        for ((strength, k), expected) in test_cases {
            assert_eq!(S::find_minimum_time(strength.to_vec(), k), expected);
        }
    }
}
