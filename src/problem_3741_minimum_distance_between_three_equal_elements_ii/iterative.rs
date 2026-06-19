pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut positions = vec![(u32::MAX, u32::MAX); nums.len()].into_boxed_slice();
        let mut result = u32::MAX;

        (0_u32..).zip(nums).for_each(|(i, num)| {
            let prev = &mut positions[num.cast_unsigned() as usize - 1];

            if let Some(distance) = i.checked_sub(prev.0) {
                result = result.min(distance);
            }

            prev.0 = prev.1;
            prev.1 = i;
        });

        if result < u32::MAX {
            result *= 2;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_distance(nums: Vec<i32>) -> i32 {
        Self::minimum_distance(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
