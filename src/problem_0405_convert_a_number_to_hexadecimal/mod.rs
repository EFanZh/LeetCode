pub mod cheating;
pub mod iterative;

pub trait Solution {
    #[allow(clippy::wrong_self_convention)]
    fn to_hex(num: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(-1, "ffffffff"), (0, "0"), (26, "1a")];

        for (num, expected) in test_cases.iter().copied() {
            assert_eq!(S::to_hex(num), expected);
        }
    }
}
