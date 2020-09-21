pub struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let mut visited = BTreeSet::new();
        let split = ((k + 1) as usize).min(nums.len());
        let (left, right) = nums.split_at(split);

        for &num in left {
            if visited
                .range(num.saturating_sub(t)..=num.saturating_add(t))
                .next()
                .is_some()
            {
                return true;
            } else {
                visited.insert(num);
            }
        }

        for (i, &num) in right.iter().enumerate() {
            visited.remove(&nums[i]);

            if visited
                .range(num.saturating_sub(t)..=num.saturating_add(t))
                .next()
                .is_some()
            {
                return true;
            } else {
                visited.insert(num);
            }
        }

        false
    }
}

impl super::Solution for Solution {
    fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        Self::contains_nearby_almost_duplicate(nums, k, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
