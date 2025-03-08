pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::{Ordering, Reverse};

impl Solution {
    fn partition<T, K>(values: &mut [T], mut key_fn: impl FnMut(&T) -> K) -> (usize, usize)
    where
        K: Ord,
    {
        // | less | equal | unknown | greater |

        let mut equal_start = 0;
        let mut unknown_start = 1;
        let mut greater_start = values.len();
        let target_key = key_fn(&values[0]);

        while unknown_start < greater_start {
            match key_fn(&values[unknown_start]).cmp(&target_key) {
                Ordering::Less => {
                    values.swap(equal_start, unknown_start);

                    equal_start += 1;
                    unknown_start += 1;
                }
                Ordering::Equal => unknown_start += 1,
                Ordering::Greater => {
                    greater_start -= 1;

                    values.swap(unknown_start, greater_start);
                }
            }
        }

        (equal_start, greater_start)
    }

    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types
            .into_iter()
            .map(|box_type| {
                let [count, units] = box_type.try_into().ok().unwrap();

                (units as u16, count as u16)
            })
            .collect::<Box<_>>();

        let mut box_types = box_types.as_mut();
        let mut truck_size = truck_size as u32;
        let mut result = 0;

        // Quick select.

        'outer: while !box_types.is_empty() {
            let (equal_start, equal_end) = Self::partition(box_types, |&(units, _)| Reverse(units));
            let (left, rest) = box_types.split_at_mut(equal_start);
            let left_total_count = left.iter().map(|&(_, count)| u32::from(count)).sum::<u32>();

            box_types = if left_total_count < truck_size {
                // Take all boxes in `left`.

                for &(units, count) in &*left {
                    result += u32::from(units) * u32::from(count);
                }

                truck_size -= left_total_count;

                // Take as much boxes as possible in `middle`.

                let (middle, right) = rest.split_at_mut(equal_end - equal_start);

                for &(units, count) in &*middle {
                    if u32::from(count) < truck_size {
                        result += u32::from(units) * u32::from(count);
                        truck_size -= u32::from(count);
                    } else {
                        result += u32::from(units) * truck_size;

                        break 'outer;
                    }
                }

                right
            } else {
                left
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        Self::maximum_units(box_types, truck_size)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
