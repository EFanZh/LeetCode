pub mod doubly_linked_list;

pub trait MyLinkedList {
    fn new() -> Self;
    fn get(&self, index: i32) -> i32;
    fn add_at_head(&mut self, val: i32);
    fn add_at_tail(&mut self, val: i32);
    fn add_at_index(&mut self, index: i32, val: i32);
    fn delete_at_index(&mut self, index: i32);
}

#[cfg(test)]
mod tests {
    use super::MyLinkedList;

    enum Operation {
        Get(i32, i32),
        AddAtHead(i32),
        AddAtTail(i32),
        AddAtIndex(i32, i32),
        DeleteAtIndex(i32),
    }

    pub fn run<S: MyLinkedList>() {
        let test_cases = [
            &[
                Operation::AddAtHead(1),
                Operation::AddAtTail(3),
                Operation::AddAtIndex(1, 2),
                Operation::Get(1, 2),
                Operation::DeleteAtIndex(1),
                Operation::Get(1, 3),
            ] as &[_],
            &[
                Operation::AddAtHead(7),
                Operation::AddAtHead(2),
                Operation::AddAtHead(1),
                Operation::AddAtIndex(3, 0),
                Operation::DeleteAtIndex(2),
                Operation::AddAtHead(6),
                Operation::AddAtTail(4),
                Operation::Get(4, 4),
            ],
            &[
                Operation::AddAtHead(7),
                Operation::AddAtTail(7),
                Operation::AddAtHead(9),
                Operation::AddAtTail(8),
                Operation::AddAtHead(6),
                Operation::AddAtHead(0),
                Operation::Get(5, 8),
                Operation::AddAtHead(0),
                Operation::Get(2, 6),
                Operation::Get(5, 7),
                Operation::AddAtTail(4),
            ],
            &[
                Operation::AddAtHead(2),
                Operation::DeleteAtIndex(1),
                Operation::AddAtHead(2),
                Operation::AddAtHead(7),
                Operation::AddAtHead(3),
                Operation::AddAtHead(2),
                Operation::AddAtHead(5),
                Operation::AddAtTail(5),
                Operation::Get(5, 2),
            ],
            &[Operation::AddAtTail(1), Operation::Get(0, 1)],
            &[
                Operation::AddAtIndex(0, 10),
                Operation::AddAtIndex(0, 20),
                Operation::AddAtIndex(1, 30),
                Operation::Get(0, 20),
            ],
            &[
                Operation::AddAtHead(1),
                Operation::AddAtTail(3),
                Operation::AddAtIndex(1, 2),
                Operation::Get(1, 2),
                Operation::DeleteAtIndex(0),
                Operation::Get(0, 2),
            ],
            &[
                Operation::AddAtHead(2),
                Operation::DeleteAtIndex(1),
                Operation::AddAtHead(2),
                Operation::AddAtHead(7),
                Operation::AddAtHead(3),
                Operation::AddAtHead(2),
                Operation::AddAtHead(5),
                Operation::AddAtTail(5),
                Operation::Get(5, 2),
                Operation::DeleteAtIndex(6),
                Operation::Get(5, 2),
                Operation::DeleteAtIndex(4),
                Operation::Get(5, -1),
            ],
            &[Operation::AddAtIndex(100, 4), Operation::Get(0, -1)],
        ];

        for operations in test_cases {
            let mut my_linked_list = S::new();

            for operation in operations {
                match *operation {
                    Operation::Get(index, expected) => assert_eq!(my_linked_list.get(index), expected),
                    Operation::AddAtHead(val) => my_linked_list.add_at_head(val),
                    Operation::AddAtTail(val) => my_linked_list.add_at_tail(val),
                    Operation::AddAtIndex(index, val) => my_linked_list.add_at_index(index, val),
                    Operation::DeleteAtIndex(index) => my_linked_list.delete_at_index(index),
                }
            }
        }
    }
}
