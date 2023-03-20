pub mod iterative;

pub trait Solution {
    fn restore_string(s: String, indices: Vec<i32>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("codeleet", &[4, 5, 6, 7, 0, 2, 1, 3] as &[_]), "leetcode"),
            (("abc", &[0, 1, 2]), "abc"),
        ];

        for ((s, indices), expected) in test_cases {
            assert_eq!(S::restore_string(s.to_string(), indices.to_vec()), expected);
        }
    }
}
