pub mod iterative;

pub trait Solution {
    fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4] as &[_], &[-1, 4, 5, 6] as &[_]), 13),
            ((&[1, -2, -5, 0, 10], &[0, -2, -1, -7, -4]), 20),
        ];

        for ((arr1, arr2), expected) in test_cases {
            assert_eq!(S::max_abs_val_expr(arr1.to_vec(), arr2.to_vec()), expected);
        }
    }
}
