pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    fn add_num(counts: &mut HashMap<u32, u32>, num: u32) {
        counts.entry(num).and_modify(|count| *count += 1).or_insert(0);
    }

    fn remove_num(counts: &mut HashMap<u32, u32>, num: u32) {
        let Entry::Occupied(entry) = counts.entry(num) else {
            unreachable!()
        };

        if *entry.get() == 0 {
            entry.remove();
        } else {
            *entry.into_mut() -= 1;
        }
    }

    pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let m = m as u32;
        let (left, right) = nums.split_at(k as u32 as usize - 1);
        let mut counts = HashMap::new();

        let mut sum = left.iter().fold(0, |sum, &num| {
            Self::add_num(&mut counts, num);

            sum + u64::from(num)
        });

        let mut result = 0;

        nums.iter().zip(right).for_each(|(&old, &new)| {
            Self::add_num(&mut counts, new);
            sum += u64::from(new);

            if counts.len() as u32 >= m {
                result = result.max(sum);
            }

            Self::remove_num(&mut counts, old);
            sum -= u64::from(old);
        });

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
        Self::max_sum(nums, m, k)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    #[should_panic = "internal error: entered unreachable code"]
    fn test_remove_num() {
        super::Solution::remove_num(&mut HashMap::new(), 2);
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
