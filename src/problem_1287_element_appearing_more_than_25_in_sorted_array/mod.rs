pub mod iterative;

pub trait Solution {
    fn find_special_integer(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 2, 6, 6, 6, 6, 7, 10] as &[_], 6), (&[1, 1], 1)];

        for (arr, expected) in test_cases {
            assert_eq!(S::find_special_integer(arr.to_vec()), expected);
        }
    }
}
