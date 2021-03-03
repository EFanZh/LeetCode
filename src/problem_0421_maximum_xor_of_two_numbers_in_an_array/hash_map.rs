pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut mask = 0;
        let mut set = HashSet::new();
        let mut probe = 1 << 30;

        loop {
            mask |= probe;

            set.extend(nums.iter().map(|x| x & mask));

            let maybe_result = result | probe;

            if set.iter().any(|x| set.contains(&(x ^ maybe_result))) {
                result = maybe_result;
            }

            set.clear();

            if probe == 1 {
                break;
            }

            probe >>= 1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        Self::find_maximum_xor(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
