pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn min_max(nums: &[i32]) -> Option<(i32, i32)> {
        let mut iter = nums.iter().copied();

        iter.next().map(|first| {
            let mut min = first;
            let mut max = first;

            while let Some(left) = iter.next() {
                if let Some(right) = iter.next() {
                    let (new_min, new_max) = if right < left { (right, left) } else { (left, right) };

                    min = min.min(new_min);
                    max = max.max(new_max);
                } else {
                    if left < min {
                        min = left;
                    } else if left > max {
                        max = left;
                    }

                    break;
                }
            }

            (min, max)
        })
    }

    fn counting_sort(mut nums: &mut [i32], min: i32, range: usize) {
        let mut counts = vec![0_u16; range];

        for &num in &*nums {
            counts[(num - min) as u32 as usize] += 1;
        }

        for (num, &count) in (min..).zip(&counts) {
            let (left, right) = nums.split_at_mut(usize::from(count));

            for target in left {
                *target = num;
            }

            nums = right;
        }
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        if let Some((min, max)) = Self::min_max(&nums) {
            if min != max {
                let range = (max - min + 1) as u32 as usize;
                let n = nums.len();

                if range <= n * (mem::size_of_val(&n) * 8 - 2 - n.leading_zeros() as usize) {
                    Self::counting_sort(&mut nums, min, range);
                } else {
                    nums.sort_unstable();
                }
            }
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::sort_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
