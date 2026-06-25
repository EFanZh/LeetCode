pub mod mathematical;

pub trait Solution {
    fn count_monobit(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 2), (4, 3)];

        for (n, expected) in test_cases {
            assert_eq!(S::count_monobit(n), expected);
        }
    }
}
