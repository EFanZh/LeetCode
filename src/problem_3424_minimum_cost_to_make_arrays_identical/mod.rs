pub mod greedy;

pub trait Solution {
    fn min_cost(arr: Vec<i32>, brr: Vec<i32>, k: i64) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[-7, 9, 5] as &[_], &[7, -2, -5] as &[_], 2), 13),
            ((&[2, 1], &[2, 1], 0), 0),
        ];

        for ((arr, brr, k), expected) in test_cases {
            assert_eq!(S::min_cost(arr.to_vec(), brr.to_vec(), k), expected);
        }
    }
}
