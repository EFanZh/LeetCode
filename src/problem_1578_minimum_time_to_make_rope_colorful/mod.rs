pub mod greedy;

pub trait Solution {
    fn min_cost(colors: String, needed_time: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abaac", &[1, 2, 3, 4, 5] as &[_]), 3),
            (("abc", &[1, 2, 3]), 0),
            (("aabaa", &[1, 2, 3, 4, 1]), 2),
        ];

        for ((colors, needed_time), expected) in test_cases {
            assert_eq!(S::min_cost(colors.to_string(), needed_time.to_vec()), expected);
        }
    }
}
