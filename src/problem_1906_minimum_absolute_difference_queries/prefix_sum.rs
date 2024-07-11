pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut counts = vec![[0_u32; 100]; nums.len()];
        let mut count = [0; 100];

        for (target, num) in counts.iter_mut().zip(nums) {
            count[(num - 1) as u32 as usize] += 1;

            *target = count;
        }

        queries
            .into_iter()
            .map(|query| {
                let [left, right]: [_; 2] = query.try_into().ok().unwrap();
                let left_count = counts.get((left - 1) as u32 as usize).unwrap_or(&[0; 100]);
                let right_count = &counts[right as u32 as usize];
                let mut result = u32::MAX;

                let mut iter = (0_u32..)
                    .zip(left_count.iter().zip(right_count))
                    .filter_map(|(i, (&x, &y))| (x != y).then_some(i));

                let mut prev = iter.next().unwrap();

                for i in iter {
                    result = result.min(i - prev);
                    prev = i;
                }

                result as _
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::min_difference(nums, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
