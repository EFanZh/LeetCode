pub mod mathematical;

pub trait Solution {
    fn total_money(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(4, 10), (10, 37), (20, 96)];

        for (n, expected) in test_cases {
            assert_eq!(S::total_money(n), expected);
        }
    }
}
