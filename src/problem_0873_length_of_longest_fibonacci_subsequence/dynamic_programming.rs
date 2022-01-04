pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn make_key(left: i32, right: i32) -> u64 {
        (u64::from(left as u32) << 32) | u64::from(right as u32)
    }

    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut lengths = HashMap::with_capacity(arr.len() * (arr.len() - 1) / 2);

        for (i, &right) in arr.iter().enumerate() {
            for &middle in &arr[..i] {
                let left = right - middle;

                let length = lengths
                    .remove(&Self::make_key(left, middle))
                    .map_or(0, |prev_length| prev_length + 1);

                lengths.insert(Self::make_key(middle, right), length);
            }
        }

        let result = lengths.values().copied().max().unwrap();

        if result == 0 {
            0
        } else {
            result + 2
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        Self::len_longest_fib_subseq(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
