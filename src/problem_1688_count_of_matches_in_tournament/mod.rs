pub mod mathematical;

pub trait Solution {
    fn number_of_matches(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(7, 6), (14, 13)];

        for (n, expected) in test_cases {
            assert_eq!(S::number_of_matches(n), expected);
        }
    }
}
