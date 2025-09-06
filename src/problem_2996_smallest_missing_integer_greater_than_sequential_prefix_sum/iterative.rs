pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    #[expect(clippy::maybe_infinite_iter, reason = "by design")]
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut iter = nums.iter().copied();
        let mut prev = iter.next().unwrap();
        let mut prefix_sum = prev;

        for num in iter {
            if num == prev + 1 {
                prefix_sum += num;
                prev = num;
            } else {
                break;
            }
        }

        let values = HashSet::<_>::from_iter(nums);

        (prefix_sum..).find(|x| !values.contains(x)).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn missing_integer(nums: Vec<i32>) -> i32 {
        Self::missing_integer(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
