pub mod mathematical;

pub trait Solution {
    fn count_commas(n: i64) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1_002, 3), (998, 0), (1_000_000_000_000_000, 3_998_998_998_999_005)];

        for (n, expected) in test_cases {
            assert_eq!(S::count_commas(n), expected,);
        }
    }
}
