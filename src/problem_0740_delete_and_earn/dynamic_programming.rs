pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut totals = HashMap::with_capacity(nums.len());

        for num in nums {
            totals.entry(num).and_modify(|total| *total += num).or_insert(num);
        }

        let mut counts = totals.into_iter().collect::<Vec<_>>();

        counts.sort_unstable_by_key(|&(num, _)| num);

        let (&(first_num, first_total), rest) = counts.split_first().unwrap();
        let mut passive = 0;
        let mut active = first_total;
        let mut prev = first_num;

        for &(num, total) in rest {
            let new_passive = passive.max(active);
            let new_active = if num == prev + 1 { passive } else { new_passive } + total;

            passive = new_passive;
            active = new_active;
            prev = num;
        }

        passive.max(active)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn delete_and_earn(nums: Vec<i32>) -> i32 {
        Self::delete_and_earn(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
