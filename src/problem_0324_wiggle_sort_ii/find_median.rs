pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut start = 0;
        let mut end = nums.len();
        let odd = end / 2;
        let index = |i: usize| i.checked_sub(odd).map_or_else(|| 1 + i * 2, |j| j * 2);
        let mut rng = StdRng::seed_from_u64(0);

        while end - start > 1 {
            nums.swap(index(start), index(rng.gen_range(start..end)));

            let x = nums[index(start)];
            let mut i = start;
            let mut j = start + 1;
            let mut k = end;

            // All elements in nums[0..i] > x.
            // All elements in nums[i..j] = x.
            // All elements in nums[j..k] = unknown.
            // All elements in nums[k..] < x.

            while j != k {
                match nums[index(j)].cmp(&x) {
                    Ordering::Less => {
                        k -= 1;

                        nums.swap(index(j), index(k));
                    }
                    Ordering::Equal => {
                        j += 1;
                    }
                    Ordering::Greater => {
                        nums.swap(index(i), index(j));

                        i += 1;
                        j += 1;
                    }
                }
            }

            if j < odd {
                start = j;
            } else if i > odd {
                end = i;
            } else {
                break;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn wiggle_sort(nums: &mut Vec<i32>) {
        Self::wiggle_sort(nums);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
