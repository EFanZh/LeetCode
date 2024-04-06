pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::array;

impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as u32 as usize;

        assert!(k >= 1 && k <= nums.len());

        let mut cache = [u16::MAX; 1024];
        let mut column_nums = Vec::new();

        cache[0] = 0;

        for i in 0..k {
            let mut counts = [0_u16; 1024];
            let iter = nums[i..].iter().step_by(k).map(|&x| x as u32 as usize);
            let rows = iter.len() as u16;

            for num in iter {
                let count = &mut counts[num as u32 as usize];

                if *count == 0 {
                    column_nums.push(num);
                }

                *count += 1;
            }

            let base_min_changes = cache.iter().copied().min().unwrap() + rows;

            cache = array::from_fn(|target| {
                let mut min_changes = base_min_changes;

                for &last in &column_nums {
                    min_changes = min_changes.min((rows - counts[last]).saturating_add(cache[target ^ last]));
                }

                min_changes
            });

            column_nums.clear();
        }

        i32::from(cache[0])
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        Self::min_changes(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
