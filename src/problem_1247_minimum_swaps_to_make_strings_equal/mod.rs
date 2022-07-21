pub mod greedy;

pub trait Solution {
    fn minimum_swap(s1: String, s2: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("xx", "yy"), 1), (("xy", "yx"), 2), (("xx", "xy"), -1)];

        for ((s1, s2), expected) in test_cases {
            assert_eq!(S::minimum_swap(s1.to_string(), s2.to_string()), expected);
        }
    }
}
