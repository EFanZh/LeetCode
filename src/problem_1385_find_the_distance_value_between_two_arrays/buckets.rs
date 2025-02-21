pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn div_floor(lhs: i32, rhs: i32) -> i32 {
        // Copied from standard library.

        let d = lhs / rhs;
        let r = lhs % rhs;

        if (r > 0 && rhs < 0) || (r < 0 && rhs > 0) {
            d - 1
        } else {
            d
        }
    }

    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let bucket_size = d + 1;
        let mut buckets = HashMap::<i32, (i32, i32)>::new();

        for value in arr2 {
            buckets
                .entry(Self::div_floor(value, bucket_size))
                .and_modify(|(min, max)| {
                    if value < *min {
                        *min = value;
                    } else if value > *max {
                        *max = value;
                    }
                })
                .or_insert((value, value));
        }

        let mut result = 0;

        for value in arr1 {
            let bucket = Self::div_floor(value, bucket_size);

            #[expect(clippy::unnecessary_map_or, reason = "compatibility")]
            if !buckets.contains_key(&bucket)
                && buckets.get(&(bucket - 1)).map_or(true, |&(_, max)| value - max > d)
                && buckets.get(&(bucket + 1)).map_or(true, |&(min, _)| min - value > d)
            {
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        Self::find_the_distance_value(arr1, arr2, d)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
