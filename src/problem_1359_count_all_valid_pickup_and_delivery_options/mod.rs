pub mod mathematical;

pub trait Solution {
    fn count_orders(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 1), (2, 6), (3, 90)];

        for (n, expected) in test_cases {
            assert_eq!(S::count_orders(n), expected);
        }
    }
}
