pub mod head_and_tail;
pub mod start_and_length;

pub trait MyCircularDeque {
    fn new(k: i32) -> Self;
    fn insert_front(&mut self, value: i32) -> bool;
    fn insert_last(&mut self, value: i32) -> bool;
    fn delete_front(&mut self) -> bool;
    fn delete_last(&mut self) -> bool;
    fn get_front(&self) -> i32;
    fn get_rear(&self) -> i32;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::MyCircularDeque;

    enum Operation {
        InsertFront(i32, bool),
        InsertLast(i32, bool),
        DeleteFront(bool),
        DeleteLast(bool),
        GetFront(i32),
        GetRear(i32),
        IsEmpty(bool),
        IsFull(bool),
    }

    const TEST_CASES: &[(i32, &[Operation])] = &[
        (
            3,
            &[
                Operation::InsertLast(1, true),
                Operation::InsertLast(2, true),
                Operation::InsertFront(3, true),
                Operation::InsertFront(4, false),
                Operation::GetRear(2),
                Operation::IsFull(true),
                Operation::DeleteLast(true),
                Operation::InsertFront(4, true),
                Operation::GetFront(4),
            ],
        ),
        (
            3,
            &[
                Operation::InsertFront(2, true),
                Operation::InsertLast(4, true),
                Operation::InsertFront(6, true),
                Operation::GetRear(4),
                Operation::InsertFront(5, false),
                Operation::GetFront(6),
                Operation::GetFront(6),
                Operation::InsertFront(6, false),
                Operation::IsFull(true),
                Operation::InsertLast(6, false),
                Operation::GetRear(4),
            ],
        ),
        (
            4,
            &[
                Operation::InsertFront(9, true),
                Operation::DeleteLast(true),
                Operation::GetRear(-1),
                Operation::GetFront(-1),
                Operation::GetFront(-1),
                Operation::DeleteFront(false),
                Operation::InsertFront(6, true),
                Operation::InsertLast(5, true),
                Operation::InsertFront(9, true),
                Operation::GetFront(9),
                Operation::InsertFront(6, true),
            ],
        ),
        (
            8,
            &[
                Operation::InsertFront(5, true),
                Operation::GetFront(5),
                Operation::IsEmpty(false),
                Operation::DeleteFront(true),
                Operation::InsertLast(3, true),
                Operation::GetRear(3),
                Operation::InsertLast(7, true),
                Operation::InsertFront(7, true),
                Operation::DeleteLast(true),
                Operation::InsertLast(4, true),
                Operation::IsEmpty(false),
            ],
        ),
        (
            3,
            &[
                Operation::InsertFront(9, true),
                Operation::GetRear(9),
                Operation::InsertFront(9, true),
                Operation::GetRear(9),
                Operation::InsertLast(5, true),
                Operation::GetFront(9),
                Operation::GetRear(5),
                Operation::GetFront(9),
                Operation::InsertLast(8, false),
                Operation::DeleteLast(true),
                Operation::GetFront(9),
            ],
        ),
        (
            71,
            &[
                Operation::InsertFront(47, true),
                Operation::DeleteLast(true),
                Operation::DeleteLast(false),
            ],
        ),
    ];

    pub fn run<Q: MyCircularDeque>() {
        for &(k, operations) in TEST_CASES {
            let mut deque = Q::new(k);

            for operation in operations {
                match *operation {
                    Operation::InsertFront(value, expected) => assert_eq!(deque.insert_front(value), expected),
                    Operation::InsertLast(value, expected) => assert_eq!(deque.insert_last(value), expected),
                    Operation::DeleteFront(expected) => assert_eq!(deque.delete_front(), expected),
                    Operation::DeleteLast(expected) => assert_eq!(deque.delete_last(), expected),
                    Operation::GetFront(expected) => assert_eq!(deque.get_front(), expected),
                    Operation::GetRear(expected) => assert_eq!(deque.get_rear(), expected),
                    Operation::IsEmpty(expected) => assert_eq!(deque.is_empty(), expected),
                    Operation::IsFull(expected) => assert_eq!(deque.is_full(), expected),
                }
            }
        }
    }
}
