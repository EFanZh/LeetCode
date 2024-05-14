pub mod btree_map;

pub trait MKAverage {
    fn new(m: i32, k: i32) -> Self;
    fn add_element(&mut self, num: i32);
    fn calculate_mk_average(&self) -> i32;
}

#[cfg(test)]
mod tests {
    use super::MKAverage;

    enum Operation {
        AddElement(i32),
        CalculateMkAverage(i32),
    }

    type TestCase<'a> = (i32, i32, &'a [Operation]);

    const EXTRA_TEST_CASE_1: TestCase = (
        6,
        2,
        &[
            Operation::AddElement(1),
            Operation::AddElement(1),
            Operation::AddElement(2),
            Operation::AddElement(2),
            Operation::AddElement(2),
            Operation::AddElement(2),
            Operation::AddElement(2),
            Operation::AddElement(2),
            Operation::AddElement(2),
            Operation::CalculateMkAverage(2),
        ],
    );

    const EXTRA_TEST_CASE_2: TestCase = (
        3,
        1,
        &[
            Operation::AddElement(8),
            Operation::AddElement(7),
            Operation::AddElement(7),
            Operation::AddElement(2),
            Operation::AddElement(3),
            Operation::AddElement(1),
            Operation::CalculateMkAverage(2),
        ],
    );

    pub fn run<M: MKAverage>() {
        let test_cases = [
            (
                3,
                1,
                &[
                    Operation::AddElement(3),
                    Operation::AddElement(1),
                    Operation::CalculateMkAverage(-1),
                    Operation::AddElement(10),
                    Operation::CalculateMkAverage(3),
                    Operation::AddElement(5),
                    Operation::AddElement(5),
                    Operation::AddElement(5),
                    Operation::CalculateMkAverage(5),
                ] as &[_],
            ),
            (
                3,
                1,
                &[
                    Operation::AddElement(58916),
                    Operation::AddElement(61899),
                    Operation::CalculateMkAverage(-1),
                    Operation::AddElement(85406),
                    Operation::AddElement(49757),
                    Operation::CalculateMkAverage(61899),
                    Operation::AddElement(27520),
                    Operation::AddElement(12303),
                    Operation::CalculateMkAverage(27520),
                    Operation::AddElement(63945),
                ],
            ),
            (
                3,
                1,
                &[
                    Operation::AddElement(3716),
                    Operation::AddElement(51094),
                    Operation::CalculateMkAverage(-1),
                    Operation::AddElement(56724),
                    Operation::AddElement(79619),
                    Operation::CalculateMkAverage(56724),
                    Operation::AddElement(99914),
                    Operation::AddElement(277),
                    Operation::CalculateMkAverage(79619),
                    Operation::AddElement(91205),
                ],
            ),
            (
                3,
                1,
                &[
                    Operation::AddElement(17612),
                    Operation::AddElement(74607),
                    Operation::CalculateMkAverage(-1),
                    Operation::AddElement(8272),
                    Operation::AddElement(33433),
                    Operation::CalculateMkAverage(33433),
                    Operation::AddElement(15456),
                    Operation::AddElement(64938),
                    Operation::CalculateMkAverage(33433),
                    Operation::AddElement(99741),
                ],
            ),
            EXTRA_TEST_CASE_1,
            EXTRA_TEST_CASE_2,
        ];

        for (m, k, operations) in test_cases {
            let mut mk_average = M::new(m, k);

            for operation in operations {
                match *operation {
                    Operation::AddElement(num) => mk_average.add_element(num),
                    Operation::CalculateMkAverage(expected) => assert_eq!(mk_average.calculate_mk_average(), expected),
                }
            }
        }
    }
}
