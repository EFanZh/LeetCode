pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn partition(values: &mut [u32], mut f: impl FnMut(u32) -> bool) -> usize {
        let mut result = 0;
        let mut iter = values.iter_mut();

        'outer: while let Some(left) = iter.next() {
            if !f(*left) {
                loop {
                    if let Some(right) = iter.next_back() {
                        if f(*right) {
                            mem::swap(left, right);

                            break;
                        }
                    } else {
                        break 'outer;
                    }
                }
            }

            result += 1;
        }

        result
    }

    fn helper(flowers: &[u32], new_flowers: u64, target: u32, full: u64, partial: u64) -> u64 {
        if flowers.is_empty() {
            return 0;
        }

        let n = flowers.len();
        let mut right_missing = 0;
        let mut i = n;

        loop {
            i = i.wrapping_sub(1);

            if let Some(&flower) = flowers.get(i) {
                right_missing += u64::from(target - flower);

                if right_missing > new_flowers {
                    break;
                }
            } else {
                return (partial * u64::from(target - 1) + full * (n - 1) as u64).max(full * n as u64);
            }
        }

        let mut left_sum = 0;
        let mut left_count = 0;
        let mut result = 0;

        while let Some(&right) = flowers.get(i) {
            i += 1;
            right_missing -= u64::from(target - right);

            let extra = new_flowers - right_missing;

            let left_min = loop {
                let new_left_sum = left_sum + u64::from(flowers[left_count]);
                let new_left_count = left_count + 1;
                let total = new_left_sum + extra;

                if new_left_count < i && total >= (u64::from(flowers[new_left_count]) + 1) * new_left_count as u64 {
                    left_sum = new_left_sum;
                    left_count = new_left_count;
                } else {
                    break total / new_left_count as u64;
                }
            };

            result = result.max(partial * left_min + full * (n - i) as u64);
        }

        result
    }

    pub fn maximum_beauty(flowers: Vec<i32>, new_flowers: i64, target: i32, full: i32, partial: i32) -> i64 {
        let mut flowers = flowers.into_iter().map(|x| x as u32).collect::<Vec<_>>();
        let new_flowers = new_flowers as u64;
        let target = target as u32;
        let full = u64::from(full as u32);
        let partial = u64::from(partial as u32);
        let split = Self::partition(&mut flowers, |x| x < target);
        let (left, right) = flowers.split_at_mut(split);
        let right_beauty = full * right.len() as u64;

        left.sort_unstable();

        (Self::helper(left, new_flowers, target, full, partial) + right_beauty) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_beauty(flowers: Vec<i32>, new_flowers: i64, target: i32, full: i32, partial: i32) -> i64 {
        Self::maximum_beauty(flowers, new_flowers, target, full, partial)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
