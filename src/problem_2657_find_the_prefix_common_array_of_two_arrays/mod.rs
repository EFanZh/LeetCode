pub mod iterative;

pub trait Solution {
    fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 2, 4] as &[_], &[3, 1, 2, 4] as &[_]), &[0, 2, 3, 4] as &[_]),
            ((&[2, 3, 1], &[3, 1, 2]), &[0, 1, 3]),
        ];

        for ((a, b), expected) in test_cases {
            assert_eq!(S::find_the_prefix_common_array(a.to_vec(), b.to_vec()), expected);
        }
    }
}
