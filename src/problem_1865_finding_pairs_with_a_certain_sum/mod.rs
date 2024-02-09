pub mod hash_map;

pub trait FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self;
    fn add(&mut self, index: i32, val: i32);
    fn count(&self, tot: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::FindSumPairs;

    enum Operation {
        Add(i32, i32),
        Count(i32, i32),
    }

    pub fn run<F: FindSumPairs>() {
        let test_cases = [(
            (&[1, 1, 2, 2, 2, 3] as &[_], &[1, 4, 5, 2, 5, 4] as &[_]),
            &[
                Operation::Count(7, 8),
                Operation::Add(3, 2),
                Operation::Count(8, 2),
                Operation::Count(4, 1),
                Operation::Add(0, 1),
                Operation::Add(1, 1),
                Operation::Count(7, 11),
            ] as &[_],
        )];

        for ((nums1, nums2), operations) in test_cases {
            let mut find_sum_pairs = F::new(nums1.to_vec(), nums2.to_vec());

            for operation in operations {
                match *operation {
                    Operation::Add(index, val) => find_sum_pairs.add(index, val),
                    Operation::Count(tot, expected) => assert_eq!(find_sum_pairs.count(tot), expected),
                }
            }
        }
    }
}
