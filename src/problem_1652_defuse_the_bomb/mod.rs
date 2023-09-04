pub mod iterative;

pub trait Solution {
    fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[5, 7, 1, 4] as &[_], 3), &[12, 10, 16, 13] as &[_]),
            ((&[1, 2, 3, 4], 0), &[0, 0, 0, 0]),
            ((&[2, 4, 9, 3], -2), &[12, 5, 6, 13]),
        ];

        for ((code, k), expected) in test_cases {
            assert_eq!(S::decrypt(code.to_vec(), k), expected);
        }
    }
}
