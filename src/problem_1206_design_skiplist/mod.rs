pub mod vanilla;
pub mod vanilla_low_memory;
pub mod vanilla_rand;

pub trait Skiplist {
    fn new() -> Self;
    fn search(&self, target: i32) -> bool;
    fn add(&mut self, num: i32);
    fn erase(&mut self, num: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Skiplist;

    enum Operation {
        Search(i32, bool),
        Add(i32),
        Erase(i32, bool),
    }

    #[allow(clippy::too_many_lines)] // Expected.
    pub fn run<S: Skiplist>() {
        let test_cases = [
            &[
                Operation::Add(1),
                Operation::Add(2),
                Operation::Add(3),
                Operation::Search(0, false),
                Operation::Add(4),
                Operation::Search(1, true),
                Operation::Erase(0, false),
                Operation::Erase(1, true),
                Operation::Search(1, false),
            ] as &[_],
            &[
                Operation::Add(9),
                Operation::Add(4),
                Operation::Add(5),
                Operation::Add(6),
                Operation::Add(9),
                Operation::Erase(2, false),
                Operation::Erase(1, false),
                Operation::Add(2),
                Operation::Search(7, false),
                Operation::Search(4, true),
                Operation::Add(5),
                Operation::Erase(6, true),
                Operation::Search(5, true),
                Operation::Add(6),
                Operation::Add(7),
                Operation::Add(4),
                Operation::Erase(3, false),
                Operation::Search(6, true),
                Operation::Erase(3, false),
                Operation::Search(4, true),
                Operation::Search(3, false),
                Operation::Search(8, false),
                Operation::Erase(7, true),
                Operation::Erase(6, true),
                Operation::Search(7, false),
                Operation::Erase(4, true),
                Operation::Add(1),
                Operation::Add(6),
                Operation::Erase(3, false),
                Operation::Add(4),
                Operation::Search(7, false),
                Operation::Search(6, true),
                Operation::Search(1, true),
                Operation::Search(0, false),
                Operation::Search(3, false),
            ],
            &[
                Operation::Add(777),
                Operation::Add(777),
                Operation::Add(777),
                Operation::Add(777),
                Operation::Add(777),
                Operation::Erase(777, true),
                Operation::Erase(777, true),
                Operation::Erase(777, true),
                Operation::Erase(777, true),
                Operation::Erase(777, true),
                Operation::Erase(777, false),
            ],
        ];

        for operations in test_cases {
            let mut skip_list = S::new();

            for operation in operations {
                match *operation {
                    Operation::Search(target, expected) => assert_eq!(skip_list.search(target), expected),
                    Operation::Add(num) => skip_list.add(num),
                    Operation::Erase(num, expected) => assert_eq!(skip_list.erase(num), expected),
                }
            }
        }
    }
}
