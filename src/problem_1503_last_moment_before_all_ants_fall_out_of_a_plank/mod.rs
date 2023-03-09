pub mod greedy;

pub trait Solution {
    fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[4, 3] as &[_], &[0, 1] as &[_]), 4),
            ((7, &[], &[0, 1, 2, 3, 4, 5, 6, 7]), 7),
            ((7, &[0, 1, 2, 3, 4, 5, 6, 7], &[]), 7),
        ];

        for ((n, left, right), expected) in test_cases {
            assert_eq!(S::get_last_moment(n, left.to_vec(), right.to_vec()), expected);
        }
    }
}
