pub mod greedy;

pub trait Solution {
    fn min_deletion(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abc", 2), 1), (("aabb", 2), 0), (("yyyzz", 1), 2)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::min_deletion(s.to_string(), k), expected);
        }
    }
}
