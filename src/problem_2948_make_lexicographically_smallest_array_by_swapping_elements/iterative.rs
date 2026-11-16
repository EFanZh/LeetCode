pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn sort_group(targets: &mut [&mut u32], buffer: &mut Vec<u32>) {
        buffer.extend(targets.iter().map(|&&mut num| num));
        targets.sort_unstable_by_key(|target| &raw const **target);

        targets
            .iter_mut()
            .zip(&*buffer)
            .for_each(|(target, &value)| **target = value);
    }

    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let limit = limit.cast_unsigned();
        let mut nums_sorted = nums.iter_mut().collect::<Box<_>>();

        nums_sorted.sort_unstable();

        let mut prev = limit.wrapping_neg();
        let mut buffer = Vec::new();
        let mut group_start = 0;

        for i in 0..nums_sorted.len() {
            let value = *nums_sorted[i];

            if value.wrapping_sub(prev) > limit {
                Self::sort_group(&mut nums_sorted[group_start..i], &mut buffer);
                buffer.clear();
                group_start = i;
            }

            prev = value;
        }

        Self::sort_group(&mut nums_sorted[group_start..], &mut buffer);

        nums.into_iter().map(u32::cast_signed).collect::<Vec<_>>()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        Self::lexicographically_smallest_array(nums, limit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
