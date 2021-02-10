pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut nums = HashMap::<i32, u16>::with_capacity(a.len() * b.len());

        for lhs in a {
            for rhs in &b {
                nums.entry(lhs + rhs).and_modify(|count| *count += 1).or_insert(1);
            }
        }

        let mut result = 0;

        for lhs in c {
            for rhs in &d {
                if let Some(&count) = nums.get(&-(lhs + rhs)) {
                    result += i32::from(count);
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        Self::four_sum_count(a, b, c, d)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
