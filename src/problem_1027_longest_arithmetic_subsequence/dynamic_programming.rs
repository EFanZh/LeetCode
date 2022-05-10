pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut cache = HashMap::<i32, HashMap<i32, u32>>::new();
        let mut buffer = Vec::new();

        for num in nums {
            for (&end, sequences) in &cache {
                let diff = num - end;

                buffer.push((diff, sequences.get(&diff).map_or(2, |&length| length + 1)));
            }

            cache.entry(num).or_default().extend(buffer.iter().copied());

            buffer.clear();
        }

        cache.values().flat_map(HashMap::values).copied().max().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        Self::longest_arith_seq_length(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
