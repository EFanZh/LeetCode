pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        const MODULUS: u64 = 1_000_000_007;

        let arr = {
            let mut arr = arr;

            arr.sort_unstable();

            arr
        };

        let mut result = 0;
        let mut cache = HashMap::<u32, u32>::with_capacity(arr.len());

        for (i, num) in arr.iter().map(|&num| num as u32).enumerate() {
            let mut count = 1;

            for lhs in arr[..i].iter().map(|&num| num as u32) {
                if num % lhs == 0 {
                    let rhs = num / lhs;

                    match lhs.cmp(&rhs) {
                        Ordering::Less => {
                            if let Some(&rhs_count) = cache.get(&rhs) {
                                count += (u64::from(cache[&lhs]) * u64::from(rhs_count) * 2) % MODULUS;
                            }
                        }
                        Ordering::Equal => {
                            count += u64::from(cache[&lhs]).pow(2) % MODULUS;

                            break;
                        }
                        Ordering::Greater => break,
                    }
                }
            }

            count %= MODULUS;

            cache.insert(num, count as _);

            result += count;
        }

        (result % MODULUS) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        Self::num_factored_binary_trees(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
