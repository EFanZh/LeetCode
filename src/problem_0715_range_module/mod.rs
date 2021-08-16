pub mod btree_map;

pub trait RangeModule {
    fn new() -> Self;
    fn add_range(&mut self, left: i32, right: i32);
    fn query_range(&self, left: i32, right: i32) -> bool;
    fn remove_range(&mut self, left: i32, right: i32);
}

#[cfg(test)]
mod tests {
    use super::RangeModule;

    enum Operation {
        Add(i32, i32),
        Query((i32, i32), bool),
        Remove(i32, i32),
    }

    pub fn run<R: RangeModule>() {
        let test_cases = [
            &[
                Operation::Add(10, 20),
                Operation::Remove(14, 16),
                Operation::Query((10, 14), true),
                Operation::Query((13, 15), false),
                Operation::Query((16, 17), true),
            ] as &[_],
            &[
                Operation::Add(6, 8),
                Operation::Remove(7, 8),
                Operation::Remove(8, 9),
                Operation::Add(8, 9),
                Operation::Remove(1, 3),
                Operation::Add(1, 8),
                Operation::Query((2, 4), true),
                Operation::Query((2, 9), true),
                Operation::Query((4, 6), true),
            ],
            &[
                Operation::Add(5, 6),
                Operation::Add(2, 8),
                Operation::Query((1, 4), false),
                Operation::Remove(2, 4),
                Operation::Query((4, 5), true),
                Operation::Remove(4, 6),
                Operation::Add(5, 9),
                Operation::Query((5, 6), true),
                Operation::Remove(6, 7),
            ],
            &[
                Operation::Query((21, 34), false),
                Operation::Query((27, 87), false),
                Operation::Add(44, 53),
                Operation::Add(69, 89),
                Operation::Query((23, 26), false),
                Operation::Query((80, 84), true),
                Operation::Query((11, 12), false),
                Operation::Remove(86, 91),
                Operation::Add(87, 94),
                Operation::Remove(34, 52),
                Operation::Add(1, 59),
                Operation::Remove(62, 96),
                Operation::Remove(34, 83),
                Operation::Query((11, 59), false),
                Operation::Query((59, 79), false),
                Operation::Query((1, 13), true),
                Operation::Query((21, 23), true),
                Operation::Remove(9, 61),
                Operation::Add(17, 21),
                Operation::Remove(4, 8),
                Operation::Query((19, 25), false),
                Operation::Add(71, 98),
                Operation::Add(23, 94),
                Operation::Remove(58, 95),
                Operation::Query((12, 78), false),
                Operation::Remove(46, 47),
                Operation::Remove(50, 70),
                Operation::Remove(84, 91),
                Operation::Add(51, 63),
                Operation::Remove(26, 64),
                Operation::Add(38, 87),
                Operation::Query((41, 84), true),
                Operation::Query((19, 21), true),
                Operation::Query((18, 56), false),
                Operation::Query((23, 39), false),
                Operation::Query((29, 95), false),
                Operation::Add(79, 100),
                Operation::Remove(76, 82),
                Operation::Add(37, 55),
                Operation::Add(30, 97),
                Operation::Add(1, 36),
                Operation::Query((18, 96), true),
                Operation::Remove(45, 86),
                Operation::Add(74, 92),
                Operation::Query((27, 78), false),
                Operation::Add(31, 35),
                Operation::Query((87, 91), true),
                Operation::Remove(37, 84),
                Operation::Remove(26, 57),
                Operation::Add(65, 87),
                Operation::Add(76, 91),
                Operation::Query((37, 77), false),
                Operation::Query((18, 66), false),
            ],
        ];

        for operations in test_cases {
            let mut range_module = R::new();

            for operation in operations {
                match *operation {
                    Operation::Add(left, right) => range_module.add_range(left, right),
                    Operation::Query((left, right), expected) => {
                        assert_eq!(range_module.query_range(left, right), expected);
                    }
                    Operation::Remove(left, right) => range_module.remove_range(left, right),
                }
            }
        }
    }
}
