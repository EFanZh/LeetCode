pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[expect(clippy::struct_field_names, reason = "by design")]
#[derive(Default)]
struct Counter {
    factor_2: u32,
    factor_3: u32,
    factor_5: u32,
    factor_7: u32,
}

impl Counter {
    fn offset_count<const N: u32>(&mut self, value: u32) {
        if value.is_multiple_of(2) {
            self.factor_2 = self.factor_2.wrapping_add(N);
        }

        if value.is_multiple_of(3) {
            self.factor_3 = self.factor_3.wrapping_add(N);
        }

        if value.is_multiple_of(5) {
            self.factor_5 = self.factor_5.wrapping_add(N);
        }

        if value.is_multiple_of(7) {
            self.factor_7 = self.factor_7.wrapping_add(N);
        }
    }
}

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut counter = Counter::default();
        let mut start = 0;

        for &num in &nums {
            counter.offset_count::<1>(num);

            if counter.factor_2 > 1 || counter.factor_3 > 1 || counter.factor_5 > 1 || counter.factor_7 > 1 {
                counter.offset_count::<{ u32::MAX }>(nums[start]);
                start += 1;
            }
        }

        (nums.len() - start).max(2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_length(nums: Vec<i32>) -> i32 {
        Self::max_length(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
