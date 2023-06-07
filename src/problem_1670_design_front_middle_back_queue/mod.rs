pub mod two_deques;

pub trait FrontMiddleBackQueue {
    fn new() -> Self;
    fn push_front(&mut self, val: i32);
    fn push_middle(&mut self, val: i32);
    fn push_back(&mut self, val: i32);
    fn pop_front(&mut self) -> i32;
    fn pop_middle(&mut self) -> i32;
    fn pop_back(&mut self) -> i32;
}

#[cfg(test)]
mod tests {
    use super::FrontMiddleBackQueue;

    enum Operation {
        PushFront(i32),
        PushMiddle(i32),
        PushBack(i32),
        PopFront(i32),
        PopMiddle(i32),
        PopBack(i32),
    }

    pub fn run<Q: FrontMiddleBackQueue>() {
        let test_cases = [
            &[
                Operation::PushFront(1),
                Operation::PushBack(2),
                Operation::PushMiddle(3),
                Operation::PushMiddle(4),
                Operation::PopFront(1),
                Operation::PopMiddle(3),
                Operation::PopMiddle(4),
                Operation::PopBack(2),
                Operation::PopFront(-1),
            ] as &[_],
            &[
                Operation::PushMiddle(493_299),
                Operation::PopMiddle(493_299),
                Operation::PushMiddle(75_427),
                Operation::PopMiddle(75_427),
                Operation::PushFront(753_523),
                Operation::PushMiddle(677_444),
                Operation::PushMiddle(431_158),
                Operation::PopMiddle(431_158),
                Operation::PopMiddle(677_444),
                Operation::PushBack(47_949),
                Operation::PopMiddle(753_523),
            ],
            &[
                Operation::PushFront(1),
                Operation::PushFront(2),
                Operation::PushFront(3),
                Operation::PushFront(4),
                Operation::PopBack(1),
                Operation::PopBack(2),
                Operation::PopBack(3),
                Operation::PopBack(4),
            ],
            &[
                Operation::PopMiddle(-1),
                Operation::PushMiddle(5_422),
                Operation::PushMiddle(532_228),
                Operation::PopBack(5_422),
                Operation::PushMiddle(206_486),
                Operation::PushBack(351_927),
                Operation::PopFront(206_486),
                Operation::PopFront(532_228),
                Operation::PushMiddle(73_577),
                Operation::PushMiddle(785_914),
                Operation::PushMiddle(765_630),
                Operation::PopMiddle(765_630),
                Operation::PushMiddle(208_060),
                Operation::PopMiddle(208_060),
                Operation::PushMiddle(592_866),
            ],
        ];

        for operations in test_cases {
            let mut front_middle_back_queue = Q::new();

            for operation in operations {
                match *operation {
                    Operation::PushFront(val) => front_middle_back_queue.push_front(val),
                    Operation::PushMiddle(val) => front_middle_back_queue.push_middle(val),
                    Operation::PushBack(val) => front_middle_back_queue.push_back(val),
                    Operation::PopFront(expected) => assert_eq!(front_middle_back_queue.pop_front(), expected),
                    Operation::PopMiddle(expected) => assert_eq!(front_middle_back_queue.pop_middle(), expected),
                    Operation::PopBack(expected) => assert_eq!(front_middle_back_queue.pop_back(), expected),
                }
            }
        }
    }
}
