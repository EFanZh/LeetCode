pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if let Some(mut first) = nums.first() {
            let mut deduped = 1;
            let mut i = 1;

            while let Some(value) = nums.get(i) {
                if value != first {
                    nums.swap(deduped, i);
                    first = &nums[deduped];
                    deduped += 1;
                }

                i += 1;
            }

            nums.truncate(deduped);
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
