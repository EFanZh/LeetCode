pub mod binary_heap;

pub trait ExamRoom {
    fn new(n: i32) -> Self;
    fn seat(&mut self) -> i32;
    fn leave(&mut self, p: i32);
}

#[cfg(test)]
mod tests {
    use super::ExamRoom;

    enum Operation {
        Seat(i32),
        Leave(i32),
    }

    #[allow(clippy::too_many_lines)]
    pub fn run<E: ExamRoom>() {
        let test_cases = [
            (
                1,
                &[
                    Operation::Seat(0),
                    Operation::Leave(0),
                    Operation::Seat(0),
                    Operation::Leave(0),
                ] as &[_],
            ),
            (
                10,
                &[
                    Operation::Seat(0),
                    Operation::Seat(9),
                    Operation::Seat(4),
                    Operation::Seat(2),
                    Operation::Leave(4),
                    Operation::Seat(5),
                ],
            ),
            (
                10,
                &[
                    Operation::Seat(0),
                    Operation::Seat(9),
                    Operation::Seat(4),
                    Operation::Leave(0),
                    Operation::Leave(4),
                    Operation::Seat(0),
                    Operation::Seat(4),
                    Operation::Seat(2),
                    Operation::Seat(6),
                    Operation::Seat(1),
                    Operation::Seat(3),
                    Operation::Seat(5),
                    Operation::Seat(7),
                    Operation::Seat(8),
                    Operation::Leave(0),
                    Operation::Leave(4),
                    Operation::Seat(0),
                    Operation::Seat(4),
                    Operation::Leave(7),
                    Operation::Seat(7),
                    Operation::Leave(3),
                    Operation::Seat(3),
                    Operation::Leave(3),
                    Operation::Seat(3),
                    Operation::Leave(9),
                    Operation::Seat(9),
                    Operation::Leave(0),
                    Operation::Leave(8),
                    Operation::Seat(0),
                    Operation::Seat(8),
                    Operation::Leave(0),
                    Operation::Leave(8),
                    Operation::Seat(0),
                    Operation::Seat(8),
                    Operation::Leave(2),
                ],
            ),
            (
                10,
                &[
                    Operation::Seat(0),
                    Operation::Seat(9),
                    Operation::Leave(9),
                    Operation::Seat(9),
                    Operation::Seat(4),
                ],
            ),
            (
                3,
                &[
                    Operation::Seat(0),
                    Operation::Leave(0),
                    Operation::Seat(0),
                    Operation::Leave(0),
                ],
            ),
            (
                100,
                &[
                    Operation::Seat(0),
                    Operation::Seat(99),
                    Operation::Seat(49),
                    Operation::Seat(74),
                    Operation::Seat(24),
                    Operation::Seat(12),
                    Operation::Leave(49),
                    Operation::Seat(49),
                    Operation::Seat(36),
                    Operation::Seat(61),
                ],
            ),
            (
                4,
                &[
                    Operation::Seat(0),
                    Operation::Seat(3),
                    Operation::Seat(1),
                    Operation::Seat(2),
                    Operation::Leave(1),
                    Operation::Leave(3),
                    Operation::Seat(1),
                ],
            ),
            (
                9,
                &[
                    Operation::Seat(0),
                    Operation::Seat(8),
                    Operation::Seat(4),
                    Operation::Seat(2),
                    Operation::Seat(6),
                    Operation::Seat(1),
                    Operation::Leave(8),
                    Operation::Leave(2),
                    Operation::Leave(1),
                    Operation::Leave(6),
                ],
            ),
            (
                4,
                &[
                    Operation::Seat(0),
                    Operation::Seat(3),
                    Operation::Seat(1),
                    Operation::Seat(2),
                    Operation::Leave(3),
                    Operation::Seat(3),
                    Operation::Leave(3),
                    Operation::Leave(0),
                    Operation::Leave(1),
                ],
            ),
            (
                7,
                &[
                    Operation::Seat(0),
                    Operation::Seat(6),
                    Operation::Seat(3),
                    Operation::Seat(1),
                ],
            ),
        ];

        for (n, operations) in test_cases {
            let mut exam_room = E::new(n);

            for operation in operations {
                match *operation {
                    Operation::Seat(expected) => assert_eq!(exam_room.seat(), expected),
                    Operation::Leave(p) => exam_room.leave(p),
                }
            }
        }
    }
}
