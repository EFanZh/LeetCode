pub mod iterative;

pub trait LockingTree {
    fn new(parent: Vec<i32>) -> Self;
    fn lock(&mut self, num: i32, user: i32) -> bool;
    fn unlock(&mut self, num: i32, user: i32) -> bool;
    fn upgrade(&mut self, num: i32, user: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::LockingTree;

    enum Operation {
        Lock((i32, i32), bool),
        Unlock((i32, i32), bool),
        Upgrade((i32, i32), bool),
    }

    pub fn run<T: LockingTree>() {
        let test_cases = [
            (
                &[-1, 0, 0, 1, 1, 2, 2] as &[_],
                &[
                    Operation::Lock((2, 2), true),
                    Operation::Unlock((2, 3), false),
                    Operation::Unlock((2, 2), true),
                    Operation::Lock((4, 5), true),
                    Operation::Upgrade((0, 1), true),
                    Operation::Lock((0, 1), false),
                ] as &[_],
            ),
            (
                &[-1, 0, 3, 1, 0],
                &[
                    Operation::Upgrade((4, 5), false),
                    Operation::Upgrade((3, 8), false),
                    Operation::Unlock((0, 7), false),
                    Operation::Lock((2, 7), true),
                    Operation::Upgrade((4, 6), false),
                ],
            ),
            (
                &[-1, 0, 8, 0, 7, 4, 2, 3, 3, 1],
                &[
                    Operation::Upgrade((8, 39), false),
                    Operation::Upgrade((5, 28), false),
                    Operation::Upgrade((6, 33), false),
                    Operation::Upgrade((9, 24), false),
                    Operation::Lock((5, 22), true),
                    Operation::Upgrade((1, 3), false),
                    Operation::Lock((5, 20), false),
                    Operation::Upgrade((0, 38), true),
                    Operation::Lock((5, 14), true),
                    Operation::Lock((6, 34), true),
                    Operation::Lock((6, 28), false),
                    Operation::Upgrade((3, 23), false),
                    Operation::Upgrade((4, 45), false),
                    Operation::Upgrade((8, 7), false),
                    Operation::Upgrade((2, 18), false),
                    Operation::Lock((3, 35), true),
                    Operation::Upgrade((2, 16), false),
                    Operation::Lock((3, 21), false),
                    Operation::Upgrade((1, 41), false),
                    Operation::Unlock((5, 22), false),
                ],
            ),
        ];

        for (parent, operations) in test_cases {
            let mut locking_tree = T::new(parent.to_vec());

            for operation in operations {
                match *operation {
                    Operation::Lock((num, user), expected) => assert_eq!(locking_tree.lock(num, user), expected),
                    Operation::Unlock((num, user), expected) => assert_eq!(locking_tree.unlock(num, user), expected),
                    Operation::Upgrade((num, user), expected) => assert_eq!(locking_tree.upgrade(num, user), expected),
                }
            }
        }
    }
}
