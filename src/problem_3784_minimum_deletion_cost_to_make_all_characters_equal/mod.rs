pub mod greedy;

pub trait Solution {
    fn min_cost(s: String, cost: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("aabaac", &[1, 2, 3, 4, 1, 10] as &[_]), 11),
            (("abc", &[10, 5, 8]), 13),
            (("zzzzz", &[67, 67, 67, 67, 67]), 0),
        ];

        for ((s, cost), expected) in test_cases {
            assert_eq!(S::min_cost(s.to_string(), cost.to_vec()), expected,);
        }
    }
}
