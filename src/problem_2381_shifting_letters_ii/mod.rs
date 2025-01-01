pub mod sweep_line;

pub trait Solution {
    fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abc", &[[0, 1, 0], [1, 2, 1], [0, 2, 1]] as &[_]), "ace"),
            (("dztz", &[[0, 0, 0], [1, 1, 1]]), "catz"),
        ];

        for ((s, shifts), expected) in test_cases {
            assert_eq!(
                S::shifting_letters(s.to_string(), shifts.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
