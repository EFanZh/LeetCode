pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut buckets = HashMap::<i32, i32>::new();
        let mut non_zeros = 0;
        let mut total_state = false;
        let mut half_state = false;

        for value in arr {
            let num = value.rem_euclid(k);

            if num == 0 {
                total_state = !total_state;
            } else {
                let other = k - num;

                let (key, diff) = match num.cmp(&other) {
                    Ordering::Less => (num, 1),
                    Ordering::Equal => {
                        half_state = !half_state;

                        continue;
                    }
                    Ordering::Greater => (other, -1),
                };

                let old_value = buckets.entry(key).or_insert(0);

                if *old_value == 0 {
                    non_zeros += 1;
                }

                *old_value += diff;

                if *old_value == 0 {
                    non_zeros -= 1;
                }
            }
        }

        non_zeros == 0 && !total_state && !half_state
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        Self::can_arrange(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
