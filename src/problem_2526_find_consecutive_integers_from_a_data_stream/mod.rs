pub mod greedy;

pub trait DataStream {
    fn new(value: i32, k: i32) -> Self;
    fn consec(&mut self, num: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::DataStream;

    pub fn run<S: DataStream>() {
        let test_cases = [((4, 3), &[(4, false), (4, false), (4, true), (3, false)] as &[_])];

        for ((value, k), operations) in test_cases {
            let mut data_stream = S::new(value, k);

            for &(num, expected) in operations {
                assert_eq!(data_stream.consec(num), expected);
            }
        }
    }
}
