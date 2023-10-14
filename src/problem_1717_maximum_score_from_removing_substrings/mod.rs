pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn maximum_gain(s: String, x: i32, y: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("cdbcbbaaabab", 4, 5), 19), (("aabbaaxybbaabb", 5, 4), 20)];

        for ((s, x, y), expected) in test_cases {
            assert_eq!(S::maximum_gain(s.to_string(), x, y), expected);
        }
    }
}
