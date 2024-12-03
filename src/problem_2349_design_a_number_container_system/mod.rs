pub mod hash_map_and_btree_set;

pub trait NumberContainers {
    fn new() -> Self;
    fn change(&mut self, index: i32, number: i32);
    fn find(&self, number: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::NumberContainers;

    enum Operation {
        Change(i32, i32),
        Find(i32, i32),
    }

    pub fn run<N: NumberContainers>() {
        let test_cases = [
            &[
                Operation::Find(10, -1),
                Operation::Change(2, 10),
                Operation::Change(1, 10),
                Operation::Change(3, 10),
                Operation::Change(5, 10),
                Operation::Find(10, 1),
                Operation::Change(1, 20),
                Operation::Find(10, 2),
            ] as &[_],
            &[
                Operation::Change(1, 10),
                Operation::Find(10, 1),
                Operation::Change(1, 20),
                Operation::Find(10, -1),
                Operation::Find(20, 1),
                Operation::Find(30, -1),
            ],
        ];

        for operations in test_cases {
            let mut number_containers = N::new();

            for operation in operations {
                match *operation {
                    Operation::Change(index, number) => number_containers.change(index, number),
                    Operation::Find(number, expected) => assert_eq!(number_containers.find(number), expected),
                }
            }
        }
    }
}
