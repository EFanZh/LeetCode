pub mod group_theory;

pub trait Solution {
    fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("5525", 9, 2), "2050"), (("74", 5, 1), "24"), (("0011", 4, 2), "0011")];

        for ((s, a, b), expected) in test_cases {
            assert_eq!(S::find_lex_smallest_string(s.to_string(), a, b), expected);
        }
    }
}
