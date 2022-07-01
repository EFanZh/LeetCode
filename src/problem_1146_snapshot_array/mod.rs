pub mod binary_search;

pub trait SnapshotArray {
    fn new(length: i32) -> Self;
    fn set(&mut self, index: i32, val: i32);
    fn snap(&mut self) -> i32;
    fn get(&self, index: i32, snap_id: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::SnapshotArray;

    enum Operation {
        Set(i32, i32),
        Snap(i32),
        Get(i32, i32, i32),
    }

    pub fn run<S: SnapshotArray>() {
        let test_cases = [
            (
                3,
                &[
                    Operation::Set(0, 5),
                    Operation::Snap(0),
                    Operation::Set(0, 6),
                    Operation::Get(0, 0, 5),
                ] as &[_],
            ),
            (
                4,
                &[
                    Operation::Snap(0),
                    Operation::Snap(1),
                    Operation::Get(3, 1, 0),
                    Operation::Set(2, 4),
                    Operation::Snap(2),
                    Operation::Set(1, 4),
                ],
            ),
            (
                1,
                &[
                    Operation::Set(0, 4),
                    Operation::Set(0, 16),
                    Operation::Set(0, 13),
                    Operation::Snap(0),
                    Operation::Get(0, 0, 13),
                ],
            ),
        ];

        for (length, operations) in test_cases {
            let mut snapshot_array = S::new(length);

            for operation in operations {
                match *operation {
                    Operation::Set(index, val) => snapshot_array.set(index, val),
                    Operation::Snap(expected) => assert_eq!(snapshot_array.snap(), expected),
                    Operation::Get(index, snap_id, expected) => {
                        assert_eq!(snapshot_array.get(index, snap_id), expected);
                    }
                }
            }
        }
    }
}
