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

    const EXTRA_TEST_CASE: &[Operation] = &[
        Operation::Add(16),
        Operation::Add(5),
        Operation::Add(14),
        Operation::Add(13),
        Operation::Add(0),
        Operation::Add(3),
        Operation::Add(12),
        Operation::Add(9),
        Operation::Add(12),
        Operation::Erase(3, true),
        Operation::Search(6, false),
        Operation::Add(7),
        Operation::Erase(0, true),
        Operation::Erase(1, false),
        Operation::Erase(10, false),
        Operation::Add(5),
        Operation::Search(12, true),
        Operation::Search(7, true),
        Operation::Search(16, true),
        Operation::Erase(7, true),
        Operation::Search(0, false),
        Operation::Add(9),
        Operation::Add(16),
        Operation::Add(3),
        Operation::Erase(2, false),
        Operation::Search(17, false),
        Operation::Add(2),
        Operation::Search(17, false),
        Operation::Erase(0, false),
        Operation::Search(9, true),
        Operation::Search(14, true),
        Operation::Erase(1, false),
        Operation::Erase(6, false),
        Operation::Add(1),
        Operation::Erase(16, true),
        Operation::Search(9, true),
        Operation::Erase(10, false),
        Operation::Erase(9, true),
        Operation::Search(2, true),
        Operation::Add(3),
        Operation::Add(16),
        Operation::Erase(15, false),
        Operation::Erase(12, true),
        Operation::Erase(7, false),
        Operation::Add(4),
        Operation::Erase(3, true),
        Operation::Add(2),
        Operation::Erase(1, true),
        Operation::Erase(14, true),
        Operation::Add(13),
        Operation::Add(12),
        Operation::Add(3),
        Operation::Search(6, false),
        Operation::Search(17, false),
        Operation::Add(2),
        Operation::Erase(3, true),
        Operation::Search(14, false),
        Operation::Add(11),
        Operation::Add(0),
        Operation::Search(13, true),
        Operation::Add(2),
        Operation::Search(1, false),
        Operation::Erase(10, false),
        Operation::Erase(17, false),
        Operation::Search(0, true),
        Operation::Search(5, true),
        Operation::Erase(8, false),
        Operation::Search(9, true),
        Operation::Add(8),
        Operation::Erase(11, true),
        Operation::Search(10, false),
        Operation::Erase(11, false),
        Operation::Search(10, false),
        Operation::Erase(9, true),
        Operation::Erase(8, true),
        Operation::Search(15, false),
        Operation::Search(14, false),
        Operation::Add(1),
        Operation::Add(6),
        Operation::Add(17),
        Operation::Add(16),
        Operation::Search(13, true),
        Operation::Search(4, true),
        Operation::Search(5, true),
        Operation::Search(4, true),
        Operation::Search(17, true),
        Operation::Search(16, true),
        Operation::Search(7, false),
        Operation::Search(14, false),
        Operation::Search(1, true),
    ];

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
            EXTRA_TEST_CASE,
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
