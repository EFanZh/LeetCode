pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut iter = nums.iter_mut();
        let mut sum = 0;
        let mut prev = mem::take(iter.next().unwrap());

        iter.for_each(|num| {
            sum += !(prev ^ *num) & 1;
            prev = mem::replace(num, sum);
        });

        queries
            .into_iter()
            .map(|query| {
                let [from, to] = query.try_into().ok().unwrap();

                nums[to.cast_unsigned() as usize] == nums[from.cast_unsigned() as usize]
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        Self::is_array_special(nums, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
