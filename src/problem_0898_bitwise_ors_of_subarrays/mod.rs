pub mod iterative;

pub trait Solution {
    fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[0] as &[_], 1), (&[1, 1, 2], 3), (&[1, 2, 4], 6)];

        for (arr, expected) in test_cases {
            assert_eq!(S::subarray_bitwise_o_rs(arr.to_vec()), expected);
        }
    }
}
