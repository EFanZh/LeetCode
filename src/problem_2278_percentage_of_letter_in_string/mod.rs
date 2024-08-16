pub mod iterative;

pub trait Solution {
    fn percentage_letter(s: String, letter: char) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("foobar", 'o'), 33), (("jjjj", 'k'), 0)];

        for ((s, letter), expected) in test_cases {
            assert_eq!(S::percentage_letter(s.to_string(), letter), expected);
        }
    }
}
