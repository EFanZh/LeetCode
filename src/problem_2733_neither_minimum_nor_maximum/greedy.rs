pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        if let Some(&[mut a, mut b, c]) = nums.get(..3) {
            if b < a {
                mem::swap(&mut a, &mut b);
            }

            a.max(b.min(c))
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        Self::find_non_min_or_max(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
