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
            ],
            &[
                Operation::Query((21, 34), false),
                Operation::Query((27, 87), false),
                Operation::Add(44, 53),
                Operation::Add(69, 89),
                Operation::Query((23, 26), false),
                Operation::Query((80, 84), true),
                Operation::Query((11, 12), false),
            ],
            &[
                Operation::Add(14, 100),
                Operation::Remove(1, 8),
                Operation::Query((77, 80), true),
                Operation::Query((8, 43), false),
                Operation::Query((4, 13), false),
                Operation::Remove(3, 9),
                Operation::Remove(45, 49),
                Operation::Remove(41, 90),
                Operation::Add(58, 79),
                Operation::Add(4, 83),
                Operation::Add(34, 39),
                Operation::Remove(84, 100),
                Operation::Add(8, 9),
                Operation::Query((32, 56), true),
                Operation::Add(35, 46),
                Operation::Add(9, 100),
                Operation::Query((85, 99), true),
                Operation::Query((23, 33), true),
                Operation::Add(10, 31),
                Operation::Remove(15, 45),
                Operation::Remove(52, 70),
                Operation::Remove(26, 42),
                Operation::Query((30, 70), false),
                Operation::Query((60, 69), false),
                Operation::Add(10, 94),
                Operation::Add(2, 89),
                Operation::Query((26, 39), true),
                Operation::Add(46, 93),
                Operation::Add(30, 83),
                Operation::Remove(42, 48),
                Operation::Add(47, 74),
                Operation::Add(39, 45),
                Operation::Query((14, 64), false),
                Operation::Remove(3, 97),
                Operation::Query((16, 34), false),
                Operation::Remove(28, 100),
                Operation::Add(19, 37),
                Operation::Add(27, 91),
                Operation::Query((55, 62), true),
                Operation::Remove(64, 65),
                Operation::Remove(2, 48),
                Operation::Add(55, 78),
                Operation::Query((21, 89), false),
                Operation::Query((31, 76), false),
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
