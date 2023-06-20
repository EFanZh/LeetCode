pub mod iterative;

pub trait Solution {
    fn find_latest_step(arr: Vec<i32>, m: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 5, 1, 2, 4] as &[_], 1), 4),
            ((&[3, 1, 5, 4, 2], 2), -1),
            ((&[1], 1), 1),
        ];

        for ((arr, m), expected) in test_cases {
            assert_eq!(S::find_latest_step(arr.to_vec(), m), expected);
        }
    }
}
