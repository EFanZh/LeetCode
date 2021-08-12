pub mod dynamic_programming;

pub trait Solution {
    fn next_greatest_letter(letters: Vec<char>, target: char) -> char;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("cfj", 'a'), 'c'),
            (("cfj", 'c'), 'f'),
            (("cfj", 'd'), 'f'),
            (("cfj", 'g'), 'j'),
            (("cfj", 'j'), 'c'),
        ];

        for ((letters, target), expected) in test_cases {
            assert_eq!(S::next_greatest_letter(letters.chars().collect(), target), expected);
        }
    }
}
