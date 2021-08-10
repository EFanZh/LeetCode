pub mod store_by_buckets;

pub trait MyHashMap {
    fn new() -> Self;
    fn put(&mut self, key: i32, value: i32);
    fn get(&self, key: i32) -> i32;
    fn remove(&mut self, key: i32);
}

#[cfg(test)]
mod tests {
    use super::MyHashMap;

    enum Operation {
        Put(i32, i32),
        Get(i32, i32),
        Remove(i32),
    }

    pub fn run<S: MyHashMap>() {
        let test_cases = [
            &[
                Operation::Put(1, 1),
                Operation::Put(2, 2),
                Operation::Get(1, 1),
                Operation::Get(3, -1),
                Operation::Put(2, 1),
                Operation::Get(2, 1),
                Operation::Remove(2),
                Operation::Get(2, -1),
            ] as &[_],
            &[
                Operation::Get(83, -1),
                Operation::Put(87, 12),
                Operation::Put(91, 90),
                Operation::Put(12, 1),
                Operation::Put(38, 14),
                Operation::Put(60, 53),
                Operation::Put(22, 1),
                Operation::Get(38, 14),
                Operation::Remove(3),
                Operation::Put(21, 67),
                Operation::Put(67, 60),
                Operation::Put(64, 31),
                Operation::Put(37, 11),
                Operation::Put(8, 30),
                Operation::Get(81, -1),
                Operation::Get(87, 12),
                Operation::Put(69, 4),
                Operation::Put(18, 82),
                Operation::Put(63, 37),
                Operation::Remove(97),
                Operation::Put(83, 48),
                Operation::Put(54, 1),
                Operation::Put(32, 81),
                Operation::Get(18, 82),
                Operation::Put(62, 37),
                Operation::Remove(21),
                Operation::Put(81, 89),
                Operation::Get(8, 30),
                Operation::Put(30, 43),
                Operation::Get(69, 4),
                Operation::Put(36, 17),
                Operation::Remove(75),
                Operation::Put(53, 51),
                Operation::Put(61, 97),
                Operation::Put(92, 45),
                Operation::Remove(39),
                Operation::Put(71, 18),
                Operation::Remove(57),
                Operation::Put(98, 10),
                Operation::Put(56, 32),
                Operation::Put(16, 52),
                Operation::Put(7, 35),
                Operation::Put(84, 11),
                Operation::Put(12, 41),
                Operation::Put(58, 6),
                Operation::Put(62, 41),
                Operation::Put(4, 44),
                Operation::Put(91, 3),
                Operation::Put(80, 45),
                Operation::Remove(36),
                Operation::Get(62, 41),
                Operation::Remove(3),
                Operation::Put(80, 46),
                Operation::Remove(18),
                Operation::Put(0, 64),
                Operation::Put(44, 29),
                Operation::Put(50, 4),
                Operation::Put(42, 7),
                Operation::Remove(83),
                Operation::Put(27, 16),
                Operation::Put(42, 8),
                Operation::Remove(72),
                Operation::Put(61, 0),
                Operation::Put(90, 85),
                Operation::Put(85, 50),
                Operation::Remove(88),
                Operation::Put(69, 23),
                Operation::Put(67, 92),
                Operation::Put(6, 46),
                Operation::Put(37, 38),
                Operation::Get(14, -1),
            ],
        ];

        for operations in test_cases {
            let mut my_hash_map = S::new();

            for operation in operations {
                match *operation {
                    Operation::Put(key, value) => my_hash_map.put(key, value),
                    Operation::Get(key, expected) => assert_eq!(my_hash_map.get(key), expected),
                    Operation::Remove(key) => my_hash_map.remove(key),
                }
            }
        }
    }
}
