pub mod mathematical;

pub trait Solution {
    fn piovt_integer(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 1), (4, -1), (8, 6)];

        for (n, expected) in test_cases {
            assert_eq!(S::piovt_integer(n), expected);
        }
    }
}
