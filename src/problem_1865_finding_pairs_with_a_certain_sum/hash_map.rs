// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::convert;

pub struct FindSumPairs {
    counts_1: HashMap<i32, i32>,
    counts_2: HashMap<i32, i32>,
    nums2: Vec<i32>,
}

impl FindSumPairs {
    fn count_nums(nums: &[i32]) -> HashMap<i32, i32> {
        let mut result = HashMap::with_capacity(nums.len());

        for &num in nums {
            result.entry(num).and_modify(|count| *count += 1).or_insert(1);
        }

        result
    }

    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let counts_1 = Self::count_nums(&convert::identity(nums1));
        let counts_2 = Self::count_nums(&nums2);

        Self {
            counts_1,
            counts_2,
            nums2,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let slot = &mut self.nums2[index as u32 as usize];

        self.counts_2.entry(*slot).and_modify(|count| *count -= 1);

        *slot += val;

        self.counts_2.entry(*slot).and_modify(|count| *count += 1).or_insert(1);
    }

    fn count(&self, tot: i32) -> i32 {
        let mut result = 0;

        for (num_1, count_1) in &self.counts_1 {
            if let Some(count_2) = self.counts_2.get(&(tot - num_1)) {
                result += count_1 * count_2;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::FindSumPairs for FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        Self::new(nums1, nums2)
    }

    fn add(&mut self, index: i32, val: i32) {
        self.add(index, val);
    }

    fn count(&self, tot: i32) -> i32 {
        self.count(tot)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::FindSumPairs>();
    }
}
