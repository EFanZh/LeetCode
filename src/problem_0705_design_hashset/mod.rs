pub mod store_by_buckets;

pub trait MyHashSet {
    fn new() -> Self;
    fn add(&mut self, key: i32);
    fn remove(&mut self, key: i32);
    fn contains(&self, key: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::MyHashSet;

    enum Operation {
        Add(i32),
        Remove(i32),
        Contains(i32, bool),
    }

    pub fn run<S: MyHashSet>() {
        let test_cases = [
            &[
                Operation::Add(1),
                Operation::Add(2),
                Operation::Contains(1, true),
                Operation::Contains(3, false),
                Operation::Add(2),
                Operation::Contains(2, true),
                Operation::Remove(2),
                Operation::Contains(2, false),
            ] as &[_],
            &[
                Operation::Add(89),
                Operation::Add(50),
                Operation::Remove(42),
                Operation::Add(51),
                Operation::Add(87),
                Operation::Add(68),
                Operation::Add(66),
                Operation::Add(35),
                Operation::Add(54),
                Operation::Add(0),
                Operation::Contains(62, false),
                Operation::Add(43),
                Operation::Add(12),
                Operation::Add(72),
                Operation::Add(17),
                Operation::Remove(89),
                Operation::Add(79),
                Operation::Remove(20),
                Operation::Contains(72, true),
                Operation::Remove(77),
                Operation::Add(91),
                Operation::Add(38),
                Operation::Add(21),
                Operation::Add(64),
                Operation::Add(80),
                Operation::Contains(35, true),
                Operation::Contains(74, false),
                Operation::Remove(40),
                Operation::Contains(25, false),
                Operation::Add(12),
                Operation::Add(37),
                Operation::Remove(20),
                Operation::Remove(48),
                Operation::Remove(34),
                Operation::Add(48),
                Operation::Contains(63, false),
                Operation::Add(28),
                Operation::Remove(33),
                Operation::Add(93),
                Operation::Remove(50),
                Operation::Add(1),
                Operation::Add(59),
                Operation::Add(14),
                Operation::Contains(20, false),
                Operation::Contains(12, true),
                Operation::Contains(14, true),
                Operation::Contains(80, true),
                Operation::Add(61),
                Operation::Add(8),
                Operation::Add(84),
                Operation::Add(46),
                Operation::Contains(61, true),
                Operation::Contains(2, false),
                Operation::Add(10),
                Operation::Remove(79),
                Operation::Add(75),
                Operation::Add(58),
                Operation::Add(85),
                Operation::Add(1),
                Operation::Add(91),
                Operation::Add(73),
                Operation::Add(83),
                Operation::Remove(12),
                Operation::Remove(93),
                Operation::Contains(8, true),
                Operation::Add(84),
                Operation::Contains(85, true),
                Operation::Contains(75, true),
                Operation::Contains(5, false),
            ],
        ];

        for operations in test_cases {
            let mut my_hash_set = S::new();

            for operation in operations {
                match *operation {
                    Operation::Add(key) => my_hash_set.add(key),
                    Operation::Remove(key) => my_hash_set.remove(key),
                    Operation::Contains(key, expected) => assert_eq!(my_hash_set.contains(key), expected),
                }
            }
        }
    }
}
