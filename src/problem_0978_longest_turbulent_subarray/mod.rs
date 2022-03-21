pub mod flip_every_second_order;

pub trait Solution {
    fn max_turbulence_size(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[9, 4, 2, 10, 7, 8, 8, 1, 9] as &[_], 5),
            (&[4, 8, 12, 16], 2),
            (&[100], 1),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::max_turbulence_size(arr.to_vec()), expected);
        }
    }
}
