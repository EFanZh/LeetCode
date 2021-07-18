pub mod trie;

pub trait MapSum {
    fn new() -> Self;
    fn insert(&mut self, key: String, val: i32);
    fn sum(&self, prefix: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::MapSum;

    enum Operation<'a> {
        Insert(&'a str, i32),
        Sum(&'a str, i32),
    }

    pub fn run<M: MapSum>() {
        let test_cases = [
            &[
                Operation::Insert("apple", 3),
                Operation::Sum("ap", 3),
                Operation::Insert("app", 2),
                Operation::Sum("ap", 5),
            ] as &[_],
            &[
                Operation::Insert("a", 3),
                Operation::Sum("ap", 0),
                Operation::Insert("b", 2),
                Operation::Sum("a", 3),
            ],
        ];

        for operations in test_cases {
            let mut map_sum = M::new();

            for operation in operations {
                match *operation {
                    Operation::Insert(key, val) => map_sum.insert(key.to_string(), val),
                    Operation::Sum(prefix, expected) => assert_eq!(map_sum.sum(prefix.to_string()), expected),
                }
            }
        }
    }
}
