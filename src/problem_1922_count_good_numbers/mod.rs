pub mod mathematical;

pub trait Solution {
    fn count_good_numbers(n: i64) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 5), (4, 400), (50, 564_908_303)];

        for (n, expected) in test_cases {
            assert_eq!(S::count_good_numbers(n), expected);
        }
    }
}
