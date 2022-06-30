pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::mem;

impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let mut cache = HashMap::from([(0, 0)]);
        let mut temp = HashMap::new();

        for rod in rods {
            let rod = rod as u32;

            temp.extend(cache.iter().map(|(&key, &value)| (key, value)));

            for (&diff, &short_length) in &cache {
                let this_length = short_length + rod;
                let other_length = short_length + diff;

                let (new_diff, new_short_length) = if other_length < this_length {
                    (this_length - other_length, other_length)
                } else {
                    (other_length - this_length, this_length)
                };

                for (diff, length) in [(new_diff, new_short_length), (diff + rod, short_length)] {
                    temp.entry(diff)
                        .and_modify(|target| *target = (*target).max(length))
                        .or_insert(length);
                }
            }

            mem::swap(&mut cache, &mut temp);
        }

        cache.get(&0).copied().unwrap_or(0) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn tallest_billboard(rods: Vec<i32>) -> i32 {
        Self::tallest_billboard(rods)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
