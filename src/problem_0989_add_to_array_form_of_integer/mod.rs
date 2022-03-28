pub mod iterative;

pub trait Solution {
    fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 0, 0] as &[_], 34), &[1, 2, 3, 4] as &[_]),
            ((&[2, 7, 4], 181), &[4, 5, 5]),
            ((&[2, 1, 5], 806), &[1, 0, 2, 1]),
            ((&[0], 10000), &[1, 0, 0, 0, 0]),
        ];

        for ((num, k), expected) in test_cases {
            assert_eq!(S::add_to_array_form(num.to_vec(), k), expected);
        }
    }
}
