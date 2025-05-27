pub mod hash_map;

pub trait FrequencyTracker {
    fn new() -> Self;
    fn add(&mut self, number: i32);
    fn delete_one(&mut self, number: i32);
    fn has_frequency(&self, frequency: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::FrequencyTracker;

    enum Operation {
        Add(i32),
        DeleteOne(i32),
        HasFrequency(i32, bool),
    }

    pub fn run<F: FrequencyTracker>() {
        let test_cases = [
            &[Operation::Add(3), Operation::Add(3), Operation::HasFrequency(2, true)] as &[_],
            &[
                Operation::Add(1),
                Operation::DeleteOne(1),
                Operation::HasFrequency(1, false),
            ],
            &[
                Operation::HasFrequency(2, false),
                Operation::Add(3),
                Operation::HasFrequency(1, true),
            ],
            &[
                Operation::HasFrequency(1, false),
                Operation::Add(3),
                Operation::HasFrequency(1, true),
                Operation::HasFrequency(1, true),
                Operation::Add(6),
                Operation::Add(2),
                Operation::Add(6),
                Operation::DeleteOne(6),
                Operation::DeleteOne(6),
                Operation::HasFrequency(2, false),
                Operation::Add(2),
                Operation::HasFrequency(2, true),
                Operation::HasFrequency(1, true),
            ],
        ];

        for operations in test_cases {
            let mut frequency_tracker = F::new();

            for operation in operations {
                match *operation {
                    Operation::Add(number) => frequency_tracker.add(number),
                    Operation::DeleteOne(number) => frequency_tracker.delete_one(number),
                    Operation::HasFrequency(frequency, expected) => {
                        assert_eq!(frequency_tracker.has_frequency(frequency), expected);
                    }
                }
            }
        }
    }
}
