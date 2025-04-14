pub mod array;
pub mod hash_map;

pub trait LUPrefix {
    fn new(n: i32) -> Self;
    fn upload(&mut self, video: i32);
    fn longest(&self) -> i32;
}

#[cfg(test)]
mod tests {
    use super::LUPrefix;

    enum Operation {
        Upload(i32),
        Longest(i32),
    }

    pub fn run<L: LUPrefix>() {
        let test_cases = [
            (
                4,
                &[
                    Operation::Upload(3),
                    Operation::Longest(0),
                    Operation::Upload(1),
                    Operation::Longest(1),
                    Operation::Upload(2),
                    Operation::Longest(3),
                ] as &[_],
            ),
            (
                10,
                &[
                    Operation::Longest(0),
                    Operation::Upload(6),
                    Operation::Longest(0),
                    Operation::Upload(10),
                    Operation::Upload(7),
                    Operation::Upload(4),
                    Operation::Longest(0),
                    Operation::Upload(2),
                    Operation::Longest(0),
                    Operation::Upload(8),
                    Operation::Upload(3),
                    Operation::Upload(1),
                    Operation::Longest(4),
                ],
            ),
        ];

        for (n, operations) in test_cases {
            let mut lu_prefix = L::new(n);

            for operation in operations {
                match *operation {
                    Operation::Upload(video) => lu_prefix.upload(video),
                    Operation::Longest(expected) => assert_eq!(lu_prefix.longest(), expected),
                }
            }
        }
    }
}
