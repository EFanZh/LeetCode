pub mod binary_heap;

pub trait KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self;
    fn add(&mut self, val: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::KthLargest;

    pub fn run<K: KthLargest>() {
        let test_cases = [
            (
                3,
                &[4, 5, 8, 2] as &[_],
                &[(3, 4), (5, 5), (10, 5), (9, 8), (4, 8)] as &[_],
            ),
            (1, &[], &[(-3, -3), (-2, -2), (-4, -2), (0, 0), (4, 4)]),
            (
                7,
                &[-10, 1, 3, 1, 4, 10, 3, 9, 4, 5, 1],
                &[
                    (3, 3),
                    (2, 3),
                    (3, 3),
                    (1, 3),
                    (2, 3),
                    (4, 3),
                    (5, 4),
                    (5, 4),
                    (6, 4),
                    (7, 5),
                    (7, 5),
                    (8, 5),
                    (2, 5),
                    (3, 5),
                    (1, 5),
                    (1, 5),
                    (1, 5),
                    (10, 6),
                    (11, 7),
                    (5, 7),
                    (6, 7),
                    (2, 7),
                    (4, 7),
                    (7, 7),
                    (8, 7),
                    (5, 7),
                    (6, 7),
                ],
            ),
        ];

        for (k, nums, adds) in test_cases {
            let mut kth_largest = K::new(k, nums.to_vec());

            for &(val, expected) in adds {
                assert_eq!(kth_largest.add(val), expected);
            }
        }
    }
}
