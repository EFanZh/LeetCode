pub mod map_to_doubly_linked_list;
pub mod map_to_doubly_linked_list_2;
pub mod map_to_doubly_linked_list_3;

pub trait LRUCache {
    fn new(capacity: i32) -> Self;
    fn get(&mut self, key: i32) -> i32;
    fn put(&mut self, key: i32, value: i32);
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    enum Operation {
        Put(i32, i32),
        Get(i32, i32),
    }

    pub fn run<S: LRUCache>() {
        use Operation::{Get, Put};

        let test_cases = [
            (
                2,
                &[Put(1, 1), Put(2, 2), Get(1, 1), Put(3, 3), Get(2, -1), Put(4, 4)] as &[_],
            ),
            (
                2,
                &[
                    Put(2, 1),
                    Put(3, 2),
                    Get(3, 2),
                    Get(2, 1),
                    Put(4, 3),
                    Get(2, 1),
                    Get(3, -1),
                    Get(4, 3),
                ],
            ),
            (
                2,
                &[
                    Put(1, 1),
                    Put(2, 2),
                    Get(1, 1),
                    Put(3, 3),
                    Get(2, -1),
                    Put(4, 4),
                    Get(1, -1),
                    Get(3, 3),
                    Get(4, 4),
                ],
            ),
        ];

        for (capacity, operations) in test_cases.iter().copied() {
            let mut cache = S::new(capacity);

            for operation in operations {
                match operation {
                    Put(key, value) => cache.put(*key, *value),
                    Get(key, expected) => assert_eq!(cache.get(*key), *expected),
                }
            }
        }
    }
}
