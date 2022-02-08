pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn combinations_2(n: u64) -> u64 {
        n * (n - 1) / 2
    }

    pub fn combinations_3(n: u64) -> u64 {
        n * (n - 1) * (n - 2) / 6
    }

    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let target = target as u32;
        let mut counts = HashMap::<u32, u64>::new();

        for num in arr {
            let num = num as u32;

            if num <= target {
                counts.entry(num).and_modify(|count| *count += 1).or_insert(1);
            }
        }

        let mut result = 0;

        for (&num_1, &count_1) in &counts {
            for (&num_2, &count_2) in &counts {
                if let Some(num_3) = target.checked_sub(num_1 + num_2) {
                    result += match num_1.cmp(&num_2) {
                        Ordering::Less => match num_2.cmp(&num_3) {
                            Ordering::Less => {
                                if let Some(count_3) = counts.get(&num_3) {
                                    count_1 * count_2 * count_3
                                } else {
                                    continue;
                                }
                            }
                            Ordering::Equal => count_1 * Self::combinations_2(count_2),
                            Ordering::Greater => continue,
                        },
                        Ordering::Equal => match num_2.cmp(&num_3) {
                            Ordering::Less => {
                                if let Some(count_3) = counts.get(&num_3) {
                                    Self::combinations_2(count_1) * count_3
                                } else {
                                    continue;
                                }
                            }
                            Ordering::Equal => Self::combinations_3(count_1),
                            Ordering::Greater => continue,
                        },
                        Ordering::Greater => continue,
                    };

                    result %= 1_000_000_007;
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        Self::three_sum_multi(arr, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
