pub mod dense_storage;

pub trait Bitset {
    fn new(size: i32) -> Self;
    fn fix(&mut self, idx: i32);
    fn unfix(&mut self, idx: i32);
    fn flip(&mut self);
    fn all(&self) -> bool;
    fn one(&self) -> bool;
    fn count(&self) -> i32;
    fn to_string(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::Bitset;

    #[allow(variant_size_differences)] // Expected.
    enum Operation {
        Fix(i32),
        Unfix(i32),
        Flip,
        All(bool),
        One(bool),
        Count(i32),
        ToString(&'static str),
    }

    const LONG_TEST_CASE: (i32, &[Operation]) = (
        90,
        &[
            Operation::Unfix(0),
            Operation::Unfix(62),
            Operation::Count(0),
            Operation::One(false),
            Operation::ToString(
                "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            ),
            Operation::Count(0),
            Operation::ToString(
                "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            ),
            Operation::Unfix(72),
            Operation::Flip,
            Operation::ToString(
                "111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111",
            ),
            Operation::Flip,
            Operation::Fix(11),
            Operation::Unfix(80),
            Operation::Unfix(80),
            Operation::ToString(
                "000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000",
            ),
            Operation::Fix(55),
            Operation::ToString(
                "000000000001000000000000000000000000000000000000000000010000000000000000000000000000000000",
            ),
            Operation::Fix(87),
            Operation::Count(3),
            Operation::Fix(61),
            Operation::Unfix(0),
            Operation::Flip,
            Operation::Unfix(67),
            Operation::Fix(40),
            Operation::Flip,
            Operation::Unfix(5),
            Operation::Count(5),
            Operation::Fix(14),
            Operation::Count(6),
            Operation::All(false),
            Operation::All(false),
            Operation::Unfix(69),
            Operation::Flip,
            Operation::All(false),
            Operation::Count(84),
            Operation::Count(84),
            Operation::One(true),
            Operation::Unfix(71),
            Operation::One(true),
            Operation::Count(83),
        ],
    );

    pub fn run<S: Bitset>() {
        let test_cases = [
            (
                5,
                &[
                    Operation::Fix(3),
                    Operation::Fix(1),
                    Operation::Flip,
                    Operation::All(false),
                    Operation::Unfix(0),
                    Operation::Flip,
                    Operation::One(true),
                    Operation::Unfix(0),
                    Operation::Count(2),
                    Operation::ToString("01010"),
                ] as &[_],
            ),
            (
                64,
                &[
                    Operation::All(false),
                    Operation::ToString("0000000000000000000000000000000000000000000000000000000000000000"),
                    Operation::ToString("0000000000000000000000000000000000000000000000000000000000000000"),
                    Operation::ToString("0000000000000000000000000000000000000000000000000000000000000000"),
                    Operation::Count(0),
                    Operation::Fix(53),
                    Operation::ToString("0000000000000000000000000000000000000000000000000000010000000000"),
                    Operation::One(true),
                    Operation::All(false),
                    Operation::Count(1),
                    Operation::All(false),
                    Operation::Fix(26),
                    Operation::One(true),
                    Operation::All(false),
                    Operation::Count(2),
                    Operation::ToString("0000000000000000000000000010000000000000000000000000010000000000"),
                    Operation::Flip,
                    Operation::ToString("1111111111111111111111111101111111111111111111111111101111111111"),
                ],
            ),
            LONG_TEST_CASE,
        ];

        for (size, operations) in test_cases {
            let mut bitset = S::new(size);

            for operation in operations {
                match *operation {
                    Operation::Fix(idx) => bitset.fix(idx),
                    Operation::Unfix(idx) => bitset.unfix(idx),
                    Operation::Flip => bitset.flip(),
                    Operation::All(expected) => assert_eq!(bitset.all(), expected),
                    Operation::One(expected) => assert_eq!(bitset.one(), expected),
                    Operation::Count(expected) => assert_eq!(bitset.count(), expected),
                    Operation::ToString(expected) => assert_eq!(bitset.to_string(), expected),
                }
            }
        }
    }
}
