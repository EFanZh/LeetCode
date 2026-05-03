pub mod iterative;

pub trait Solution {
    fn concat_hex36(n: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(13, "A91P1"), (36, "5101000")];

        for (n, expected) in test_cases {
            assert_eq!(S::concat_hex36(n), expected);
        }
    }
}
