pub mod queue;

pub trait RecentCounter {
    fn new() -> Self;
    fn ping(&mut self, t: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::RecentCounter;

    pub fn run<C: RecentCounter>() {
        let test_cases = [&[(1, 1), (100, 2), (3001, 3), (3002, 3)] as &[_]];

        for operations in test_cases {
            let mut recent_counter = C::new();

            for &(price, expected) in operations {
                assert_eq!(recent_counter.ping(price), expected);
            }
        }
    }
}
