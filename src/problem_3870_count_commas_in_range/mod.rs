pub mod mathematical;

pub trait Solution {
    fn count_commas(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1002, 3), (998, 0)];

        for (n, expected) in test_cases {
            assert_eq!(S::count_commas(n), expected);
        }
    }
}
