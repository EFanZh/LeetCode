pub mod binary_tree;
pub mod fenwick_tree;
pub mod heap_like_binary_tree;

pub trait NumArray {
    fn new(nums: Vec<i32>) -> Self;
    fn update(&mut self, i: i32, val: i32);
    fn sum_range(&self, i: i32, j: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::NumArray;

    enum Operation {
        Update(i32, i32),
        SumRange((i32, i32), i32),
    }

    pub fn run<N: NumArray>() {
        use Operation::{SumRange, Update};

        let test_cases = [
            (
                &[1, 3, 5] as &[_],
                &[SumRange((0, 2), 9), Update(1, 2), SumRange((0, 2), 8)] as &[_],
            ),
            (&[], &[]),
            (
                &[0, 9, 5, 7, 3],
                &[
                    SumRange((4, 4), 3),
                    SumRange((2, 4), 15),
                    SumRange((3, 3), 7),
                    Update(4, 5),
                    Update(1, 7),
                    Update(0, 8),
                    SumRange((1, 2), 12),
                    Update(1, 9),
                    SumRange((4, 4), 5),
                    Update(3, 4),
                ],
            ),
            (&[-1], &[SumRange((0, 0), -1), Update(0, 1), SumRange((0, 0), 1)]),
        ];

        for (nums, operations) in test_cases {
            let mut num_array = N::new(nums.to_vec());

            for operation in operations {
                match *operation {
                    Update(i, val) => num_array.update(i, val),
                    SumRange((i, j), expected) => assert_eq!(num_array.sum_range(i, j), expected),
                }
            }
        }
    }
}
