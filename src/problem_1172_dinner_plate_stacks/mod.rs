pub mod heap;

pub trait DinnerPlates {
    fn new(capacity: i32) -> Self;
    fn push(&mut self, val: i32);
    fn pop(&mut self) -> i32;
    fn pop_at_stack(&mut self, index: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::DinnerPlates;

    enum Operation {
        Push(i32),
        Pop(i32),
        PopAtStack(i32, i32),
    }

    #[allow(clippy::too_many_lines)] // Expected.
    pub fn run<D: DinnerPlates>() {
        let test_cases = [
            (
                2,
                &[
                    Operation::Push(1),
                    Operation::Push(2),
                    Operation::Push(3),
                    Operation::Push(4),
                    Operation::Push(5),
                    Operation::PopAtStack(0, 2),
                    Operation::Push(20),
                    Operation::Push(21),
                    Operation::PopAtStack(0, 20),
                    Operation::PopAtStack(2, 21),
                    Operation::Pop(5),
                    Operation::Pop(4),
                    Operation::Pop(3),
                    Operation::Pop(1),
                    Operation::Pop(-1),
                ] as &[_],
            ),
            (
                2,
                &[
                    Operation::Push(132),
                    Operation::Push(417),
                    Operation::Push(114),
                    Operation::Push(340),
                    Operation::Push(224),
                    Operation::Push(401),
                    Operation::Push(28),
                    Operation::Push(267),
                    Operation::PopAtStack(3, 267),
                    Operation::PopAtStack(2, 401),
                    Operation::Push(418),
                    Operation::Push(80),
                    Operation::Push(298),
                    Operation::Push(39),
                    Operation::Push(427),
                    Operation::Push(273),
                    Operation::Push(148),
                    Operation::Push(362),
                    Operation::Pop(362),
                    Operation::Pop(148),
                ],
            ),
            (
                1,
                &[
                    Operation::Push(1),
                    Operation::Push(2),
                    Operation::PopAtStack(1, 2),
                    Operation::Pop(1),
                    Operation::Push(1),
                    Operation::Push(2),
                    Operation::Pop(2),
                    Operation::Pop(1),
                ],
            ),
            (
                2,
                &[
                    Operation::Push(471),
                    Operation::Push(177),
                    Operation::Push(1),
                    Operation::Push(29),
                    Operation::Push(333),
                    Operation::Push(154),
                    Operation::Push(130),
                    Operation::Push(333),
                    Operation::PopAtStack(1, 29),
                    Operation::PopAtStack(0, 177),
                    Operation::PopAtStack(2, 154),
                    Operation::PopAtStack(0, 471),
                    Operation::Push(165),
                    Operation::Push(383),
                    Operation::Push(267),
                    Operation::Push(367),
                    Operation::Push(53),
                    Operation::Push(373),
                    Operation::Push(388),
                    Operation::Push(249),
                    Operation::Pop(249),
                    Operation::Pop(388),
                    Operation::Pop(373),
                    Operation::Pop(53),
                ],
            ),
            (
                2,
                &[
                    Operation::Push(1),
                    Operation::Push(2),
                    Operation::Push(3),
                    Operation::Push(4),
                    Operation::Push(7),
                    Operation::PopAtStack(8, -1),
                    Operation::Push(20),
                    Operation::Push(21),
                    Operation::PopAtStack(0, 2),
                    Operation::PopAtStack(2, 20),
                    Operation::Pop(21),
                    Operation::Pop(7),
                    Operation::Pop(4),
                    Operation::Pop(3),
                    Operation::Pop(1),
                ],
            ),
            (
                2,
                &[
                    Operation::Push(472),
                    Operation::Push(106),
                    Operation::Push(497),
                    Operation::Push(498),
                    Operation::Push(73),
                    Operation::Push(115),
                    Operation::Push(437),
                    Operation::Push(461),
                    Operation::PopAtStack(3, 461),
                    Operation::PopAtStack(3, 437),
                    Operation::PopAtStack(1, 498),
                    Operation::PopAtStack(3, -1),
                    Operation::PopAtStack(0, 106),
                    Operation::PopAtStack(2, 115),
                    Operation::PopAtStack(2, 73),
                    Operation::PopAtStack(1, 497),
                    Operation::PopAtStack(1, -1),
                    Operation::PopAtStack(3, -1),
                    Operation::Push(197),
                    Operation::Push(239),
                    Operation::Push(129),
                    Operation::Push(449),
                    Operation::Push(460),
                    Operation::Push(240),
                    Operation::Push(386),
                    Operation::Push(343),
                    Operation::Pop(343),
                    Operation::Pop(386),
                    Operation::Pop(240),
                    Operation::Pop(460),
                    Operation::Pop(449),
                    Operation::Pop(129),
                    Operation::Pop(239),
                    Operation::Pop(197),
                    Operation::Pop(472),
                    Operation::Pop(-1),
                ],
            ),
        ];

        for (length, operations) in test_cases {
            let mut dinner_plates = D::new(length);

            for operation in operations {
                match *operation {
                    Operation::Push(val) => dinner_plates.push(val),
                    Operation::Pop(expected) => assert_eq!(dinner_plates.pop(), expected),
                    Operation::PopAtStack(index, expected) => assert_eq!(dinner_plates.pop_at_stack(index), expected),
                }
            }
        }
    }
}
