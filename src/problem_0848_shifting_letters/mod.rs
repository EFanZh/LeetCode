pub mod iterative;

pub trait Solution {
    fn shifting_letters(s: String, shifts: Vec<i32>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abc", &[3, 5, 9] as &[_]), "rpl"),
            (("aaa", &[1, 2, 3]), "gfd"),
            (("ruu", &[26, 9, 17]), "rul"),
        ];

        for ((s, shifts), expected) in test_cases {
            assert_eq!(S::shifting_letters(s.to_string(), shifts.to_vec()), expected);
        }
    }
}
