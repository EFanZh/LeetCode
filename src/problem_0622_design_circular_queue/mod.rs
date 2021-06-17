pub mod head_and_tail;
pub mod start_and_length;

pub trait MyCircularQueue {
    fn new(k: i32) -> Self;
    fn en_queue(&mut self, value: i32) -> bool;
    fn de_queue(&mut self) -> bool;
    fn front(&self) -> i32;
    fn rear(&self) -> i32;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::MyCircularQueue;

    enum Operation {
        EnQueue(i32, bool),
        DeQueue(bool),
        Front(i32),
        Rear(i32),
        IsEmpty(bool),
        IsFull(bool),
    }

    const TEST_CASES: &[(i32, &[Operation])] = &[
        (
            3,
            &[
                Operation::EnQueue(1, true),
                Operation::EnQueue(2, true),
                Operation::EnQueue(3, true),
                Operation::EnQueue(4, false),
                Operation::Rear(3),
                Operation::IsFull(true),
                Operation::DeQueue(true),
                Operation::EnQueue(4, true),
                Operation::Rear(4),
            ] as &[_],
        ),
        (
            6,
            &[
                Operation::EnQueue(6, true),
                Operation::Rear(6),
                Operation::Rear(6),
                Operation::DeQueue(true),
                Operation::EnQueue(5, true),
                Operation::Rear(5),
                Operation::DeQueue(true),
                Operation::Front(-1),
                Operation::DeQueue(false),
                Operation::DeQueue(false),
                Operation::DeQueue(false),
            ],
        ),
        (
            3,
            &[
                Operation::EnQueue(1, true),
                Operation::EnQueue(2, true),
                Operation::EnQueue(3, true),
                Operation::EnQueue(4, false),
                Operation::Rear(3),
                Operation::IsFull(true),
                Operation::DeQueue(true),
                Operation::EnQueue(4, true),
                Operation::Rear(4),
            ],
        ),
        (
            2,
            &[
                Operation::EnQueue(1, true),
                Operation::EnQueue(2, true),
                Operation::DeQueue(true),
                Operation::EnQueue(3, true),
                Operation::DeQueue(true),
                Operation::EnQueue(3, true),
                Operation::DeQueue(true),
                Operation::EnQueue(3, true),
                Operation::DeQueue(true),
                Operation::Front(3),
            ],
        ),
        (
            8,
            &[
                Operation::EnQueue(3, true),
                Operation::EnQueue(9, true),
                Operation::EnQueue(5, true),
                Operation::EnQueue(0, true),
                Operation::DeQueue(true),
                Operation::DeQueue(true),
                Operation::IsEmpty(false),
                Operation::IsEmpty(false),
                Operation::Rear(0),
                Operation::Rear(0),
                Operation::DeQueue(true),
            ],
        ),
        (
            3,
            &[
                Operation::EnQueue(7, true),
                Operation::DeQueue(true),
                Operation::Front(-1),
                Operation::DeQueue(false),
                Operation::Front(-1),
                Operation::Rear(-1),
                Operation::EnQueue(0, true),
                Operation::IsFull(false),
                Operation::DeQueue(true),
                Operation::Rear(-1),
                Operation::EnQueue(3, true),
            ],
        ),
    ];

    pub fn run<Q: MyCircularQueue>() {
        for &(k, operations) in TEST_CASES {
            let mut queue = Q::new(k);

            for operation in operations {
                match *operation {
                    Operation::EnQueue(value, expected) => assert_eq!(queue.en_queue(value), expected),
                    Operation::DeQueue(expected) => assert_eq!(queue.de_queue(), expected),
                    Operation::Front(expected) => assert_eq!(queue.front(), expected),
                    Operation::Rear(expected) => assert_eq!(queue.rear(), expected),
                    Operation::IsEmpty(expected) => assert_eq!(queue.is_empty(), expected),
                    Operation::IsFull(expected) => assert_eq!(queue.is_full(), expected),
                }
            }
        }
    }
}
