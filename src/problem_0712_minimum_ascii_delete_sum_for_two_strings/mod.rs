pub mod dynamic_programming;

pub trait Solution {
    fn minimum_delete_sum(s1: String, s2: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("sea", "eat"), 231), (("delete", "leet"), 403)];

        for ((s1, s2), expected) in test_cases {
            assert_eq!(S::minimum_delete_sum(s1.to_string(), s2.to_string()), expected);
        }
    }
}
