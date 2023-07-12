pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let max = deliciousness.iter().fold(0, |max, &x| u32::max(max, x as _));
        let mut result = 0_u32;

        if max != 0 {
            let max_sum = 1_u32 << (32 - max.leading_zeros());
            let mut counts = HashMap::new();

            for x in deliciousness {
                let x = x as u32;
                let mut sum = x.next_power_of_two();

                while sum <= max_sum {
                    result += counts.get(&(sum - x)).copied().unwrap_or(0);
                    result = result.checked_sub(1_000_000_007).unwrap_or(result);

                    sum <<= 1;
                }

                counts.entry(x).and_modify(|count| *count += 1).or_insert(1);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        Self::count_pairs(deliciousness)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
