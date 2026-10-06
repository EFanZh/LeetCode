pub mod prefix_sums;

pub trait Solution {
    fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("adaa", "d", &[-1000] as &[_]), 2), (("abc", "abc", &[-1, -1, -1]), 0)];

        for ((s, chars, vals), expected) in test_cases {
            assert_eq!(
                S::maximum_cost_substring(s.to_string(), chars.to_string(), vals.to_vec()),
                expected,
            );
        }
    }
}
