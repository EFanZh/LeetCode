pub struct Solution;

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    fn permute_unique_helper(nums: &mut [(i32, i32)], base: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if let Some((mut first, rest)) = nums.split_first_mut() {
            base.push(first.0);

            if first.1 == 1 {
                Self::permute_unique_helper(rest, base, result);
            } else {
                first.1 -= 1;

                Self::permute_unique_helper(nums, base, result);

                nums.first_mut().unwrap().1 += 1;
            }

            for i in 1..nums.len() {
                nums.swap(0, i);

                let (first, rest) = nums.split_first_mut().unwrap();

                *base.last_mut().unwrap() = first.0;

                if first.1 == 1 {
                    Self::permute_unique_helper(rest, base, result);
                } else {
                    first.1 -= 1;

                    Self::permute_unique_helper(nums, base, result);

                    nums.first_mut().unwrap().1 += 1;
                }

                nums.swap(0, i);
            }

            base.pop();
        } else {
            result.push(base.clone());
        }
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut counts = HashMap::new();

        for num in nums {
            match counts.entry(num) {
                Entry::Occupied(entry) => *entry.into_mut() += 1,
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }

        let mut result = Vec::new();

        Self::permute_unique_helper(
            counts.into_iter().collect::<Box<_>>().as_mut(),
            &mut Vec::new(),
            &mut result,
        );

        result
    }
}

impl super::Solution for Solution {
    fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::permute_unique(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
