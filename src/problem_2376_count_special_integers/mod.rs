pub mod mathematical;

pub trait Solution {
    fn count_special_numbers(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (20, 19), (99, 90), (135, 110)];

        for (n, expected) in test_cases {
            assert_eq!(S::count_special_numbers(n), expected);
        }
    }
}
