pub mod binary_search;

pub trait Solution {
    fn min_time(s: String, order: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abc", &[1, 0, 2] as &[_], 2), 0),
            (("cat", &[0, 2, 1], 6), 2),
            (("xy", &[0, 1], 4), -1),
            (("ow", &[1, 0], 2), 0),
        ];

        for ((s, order, k), expected) in test_cases {
            assert_eq!(S::min_time(s.to_string(), order.to_vec(), k), expected,);
        }
    }
}
