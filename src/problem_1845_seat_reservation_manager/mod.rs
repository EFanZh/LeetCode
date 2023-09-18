pub mod binary_heap;
pub mod lazy_binary_heap;

pub trait SeatManager {
    fn new(n: i32) -> Self;
    fn reserve(&mut self) -> i32;
    fn unreserve(&mut self, seat_number: i32);
}

#[cfg(test)]
mod tests {
    use super::SeatManager;

    enum Operation {
        Reserve(i32),
        Unreserve(i32),
    }

    pub fn run<M: SeatManager>() {
        let test_cases = [
            (
                5,
                &[
                    Operation::Reserve(1),
                    Operation::Reserve(2),
                    Operation::Unreserve(2),
                    Operation::Reserve(2),
                    Operation::Reserve(3),
                    Operation::Reserve(4),
                    Operation::Reserve(5),
                    Operation::Unreserve(5),
                ] as &[_],
            ),
            (
                4,
                &[
                    Operation::Reserve(1),
                    Operation::Unreserve(1),
                    Operation::Reserve(1),
                    Operation::Reserve(2),
                    Operation::Reserve(3),
                    Operation::Unreserve(2),
                    Operation::Reserve(2),
                    Operation::Unreserve(1),
                    Operation::Reserve(1),
                    Operation::Unreserve(2),
                ],
            ),
        ];

        for (n, operations) in test_cases {
            let mut seat_manager = M::new(n);

            for operation in operations {
                match *operation {
                    Operation::Reserve(expected) => assert_eq!(seat_manager.reserve(), expected),
                    Operation::Unreserve(seat_number) => seat_manager.unreserve(seat_number),
                }
            }
        }
    }
}
