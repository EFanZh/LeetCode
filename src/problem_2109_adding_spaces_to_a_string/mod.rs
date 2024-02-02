pub mod iterative;

pub trait Solution {
    fn add_spaces(s: String, indices: Vec<i32>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                ("LeetcodeHelpsMeLearn", &[8, 13, 15] as &[_]),
                "Leetcode Helps Me Learn",
            ),
            (("icodeinpython", &[1, 5, 7, 9]), "i code in py thon"),
            (("spacing", &[0, 1, 2, 3, 4, 5, 6]), " s p a c i n g"),
        ];

        for ((s, indices), expected) in test_cases {
            assert_eq!(S::add_spaces(s.to_string(), indices.to_vec()), expected);
        }
    }
}
