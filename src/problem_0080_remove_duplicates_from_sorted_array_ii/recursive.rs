pub struct Solution;

impl Solution {
    fn remove_duplicates_single(nums: &mut [i32], deduped: usize, prev: i32, i: usize) -> usize {
        nums.get(i).copied().map_or(deduped, |current| {
            nums.swap(deduped, i);

            if current == prev {
                Self::remove_duplicates_double(nums, deduped + 1, prev, i + 1)
            } else {
                Self::remove_duplicates_single(nums, deduped + 1, current, i + 1)
            }
        })
    }

    fn remove_duplicates_double(nums: &mut [i32], deduped: usize, prev: i32, i: usize) -> usize {
        nums.get(i).copied().map_or(deduped, |current| {
            if current == prev {
                Self::remove_duplicates_double(nums, deduped, prev, i + 1)
            } else {
                nums.swap(deduped, i);

                Self::remove_duplicates_single(nums, deduped + 1, current, i + 1)
            }
        })
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if let Some(first) = nums.first().copied() {
            let new_length = Self::remove_duplicates_single(nums, 1, first, 1);

            nums.truncate(new_length);
        }

        nums.len() as _
    }
}

impl super::Solution for Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        Self::remove_duplicates(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
