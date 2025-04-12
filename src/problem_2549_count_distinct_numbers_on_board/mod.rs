pub mod mathematical;

pub trait Solution {
    fn distinct_integers(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 1), (2, 1), (3, 2), (4, 3), (5, 4)];

        for (n, expected) in test_cases {
            assert_eq!(S::distinct_integers(n), expected);
        }
    }
}
