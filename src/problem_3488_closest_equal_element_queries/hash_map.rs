pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    fn assign_min(target: &mut i32, value: u32) {
        *target = target.cast_unsigned().min(value).cast_signed();
    }

    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut queries = queries;
        let n = nums.len();
        let mut ranges = HashMap::<i32, (u32, u32)>::new();
        let mut i = 0;

        while let Some(&num) = nums.get(i) {
            let i_u32 = i as u32;

            match ranges.entry(num) {
                Entry::Occupied(occupied_entry) => {
                    let range = occupied_entry.into_mut();
                    let distance_1 = i_u32 - range.1;
                    let distance_2 = range.0 + n as u32 - i_u32;

                    Self::assign_min(&mut nums[range.1 as usize], distance_1);
                    Self::assign_min(&mut nums[range.0 as usize], distance_2);
                    nums[i] = distance_1.min(distance_2).cast_signed();
                    range.1 = i_u32;
                }
                Entry::Vacant(vacant_entry) => {
                    nums[i] = -1;
                    vacant_entry.insert((i_u32, i_u32));
                }
            }

            i += 1;
        }

        drop(ranges);

        for query in &mut queries {
            *query = nums[query.cast_unsigned() as usize];
        }

        queries
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        Self::solve_queries(nums, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
