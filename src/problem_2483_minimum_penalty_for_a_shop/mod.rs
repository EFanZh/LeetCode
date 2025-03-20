pub mod iterative;

pub trait Solution {
    fn best_closing_time(customers: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("YYNY", 2), ("NNNNN", 0), ("YYYY", 4)];

        for (customers, expected) in test_cases {
            assert_eq!(S::best_closing_time(customers.to_string()), expected);
        }
    }
}
