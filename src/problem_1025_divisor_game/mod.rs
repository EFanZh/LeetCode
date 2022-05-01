pub mod mathematical;

pub trait Solution {
    fn divisor_game(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(2, true), (3, false)];

        for (n, expected) in test_cases {
            assert_eq!(S::divisor_game(n), expected);
        }
    }
}
