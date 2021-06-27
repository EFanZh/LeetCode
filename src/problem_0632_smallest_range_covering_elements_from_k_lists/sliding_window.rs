pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let k = nums.len();
        let mut events = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            for &x in num {
                events
                    .entry(x)
                    .and_modify(|v| Vec::push(v, i))
                    .or_insert_with(|| vec![i]);
            }
        }

        let mut events = events.into_iter().collect::<Vec<_>>();

        events.sort_unstable_by_key(|&(x, _)| x);

        let mut best_low = 0;
        let mut best_high = i32::MAX;
        let mut left_iter = events.iter();
        let mut counts = vec![0; nums.len()];
        let mut covered = 0;

        for (high, lists) in &events {
            for &list in lists {
                let count = &mut counts[list];

                if *count == 0 {
                    covered += 1;
                }

                *count += 1;
            }

            if covered == k {
                loop {
                    let (low, lists) = left_iter.next().unwrap();

                    for &list in lists {
                        let count = &mut counts[list];

                        *count -= 1;

                        if *count == 0 {
                            covered -= 1;
                        }
                    }

                    if covered != k {
                        if high - low < best_high - best_low {
                            best_low = *low;
                            best_high = *high;
                        }

                        break;
                    }
                }
            }
        }

        vec![best_low, best_high]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        Self::smallest_range(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
