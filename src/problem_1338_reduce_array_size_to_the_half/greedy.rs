pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut counts = HashMap::new();

        for value in arr {
            counts.entry(value).and_modify(|count| *count += 1).or_insert(0);
        }

        let mut count_counts = vec![0; n];

        for count in counts.into_values() {
            count_counts[count] += 1;
        }

        let mut i = n;
        let mut required = (n + 1) / 2;
        let mut result = 0;

        loop {
            let count = i;

            i -= 1;

            let count_count = count_counts[i];
            let total = count * count_count;

            if total >= required {
                result += (required + count - 1) / count;

                break;
            }

            required -= total;
            result += count_count;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_set_size(arr: Vec<i32>) -> i32 {
        Self::min_set_size(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
