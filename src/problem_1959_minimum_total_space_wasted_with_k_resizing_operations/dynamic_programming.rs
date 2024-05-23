pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut cache = vec![0; n].into_boxed_slice();
        let k = k as u32 as usize;

        let mut max = 0;
        let mut count = 0;
        let mut total = 0;

        for (target, &num) in cache.iter_mut().zip(&nums) {
            let num = num as u32;

            max = u32::max(max, num);
            count += 1;
            *target = max * count;

            total += num;
        }

        for resize_count in 1..=k {
            for last_index in (resize_count..n).rev() {
                let mut max = 0;
                let mut count = 0;
                let mut min_capacity = u32::MAX;

                for i in (resize_count..=last_index).rev() {
                    max = u32::max(max, nums[i] as _);
                    count += 1;
                    min_capacity = u32::min(min_capacity, cache[i - 1] + max * count);
                }

                cache[last_index] = min_capacity;
            }
        }

        (cache.last().unwrap() - total) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
        Self::min_space_wasted_k_resizing(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
