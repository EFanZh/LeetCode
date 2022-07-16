pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::hash_map::{Entry, OccupiedEntry};
use std::collections::HashMap;

impl Solution {
    fn unwrap_occupied<K, V>(entry: Entry<K, V>) -> OccupiedEntry<K, V> {
        match entry {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(_) => panic!(),
        }
    }

    fn fetch_add(value: &mut usize) -> usize {
        let result = *value;

        *value += 1;

        result
    }

    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut result = 0;
        let mut window = HashMap::<_, (u16, u16)>::new();
        let mut min_start = 0;
        let mut max_start = 0;

        for &num in &nums {
            window
                .entry(num)
                .and_modify(|count| {
                    count.0 += 1;
                    count.1 += 1;
                })
                .or_insert((1, 1));

            match window.len().cmp(&k) {
                Ordering::Less => {}
                Ordering::Equal => {
                    loop {
                        let entry = Self::unwrap_occupied(window.entry(nums[max_start]));

                        if entry.get().1 == 1 {
                            break;
                        }

                        entry.into_mut().1 -= 1;
                        max_start += 1;
                    }

                    result += max_start - min_start + 1;
                }
                Ordering::Greater => {
                    loop {
                        let entry = Self::unwrap_occupied(window.entry(nums[Self::fetch_add(&mut min_start)]));

                        if entry.get().0 == 1 {
                            entry.remove();

                            break;
                        }

                        entry.into_mut().0 -= 1;
                    }

                    loop {
                        match window.entry(nums[max_start]) {
                            Entry::Occupied(entry) => {
                                if entry.get().1 == 1 {
                                    break;
                                }

                                entry.into_mut().1 -= 1;
                                max_start += 1;
                            }
                            Entry::Vacant(_) => max_start += 1,
                        }
                    }

                    result += max_start - min_start + 1;
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        Self::subarrays_with_k_distinct(nums, k)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_unwrap_occupied_success() {
        super::Solution::unwrap_occupied(HashMap::from([(2, (3_u16, 5_u16))]).entry(2));
    }

    #[test]
    #[should_panic]
    fn test_unwrap_occupied_failure() {
        super::Solution::unwrap_occupied(HashMap::from([(2, (3_u16, 5_u16))]).entry(5));
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
