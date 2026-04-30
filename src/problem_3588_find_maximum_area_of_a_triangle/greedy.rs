pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::mem;

impl Solution {
    fn min_max(mut iter: impl Iterator<Item = u32>) -> Option<(u32, u32)> {
        iter.next().and_then(|mut min| {
            iter.next().map(|mut max| {
                if max < min {
                    mem::swap(&mut min, &mut max);
                }

                while let Some(mut first) = iter.next() {
                    if let Some(mut second) = iter.next() {
                        if second < first {
                            mem::swap(&mut first, &mut second);
                        }

                        if first < min {
                            min = first;
                        }

                        if second > max {
                            max = second;
                        }
                    } else {
                        if first < min {
                            min = first;
                        } else if first > max {
                            max = first;
                        }

                        break;
                    }
                }

                (min, max)
            })
        })
    }

    fn check(buckets: &HashMap<u32, Vec<u32>>) -> Option<u64> {
        Self::min_max(buckets.keys().copied()).and_then(|(major_min, major_max)| {
            buckets
                .iter()
                .filter_map(|(major, minors)| {
                    Self::min_max(minors.iter().copied()).map(|(minor_min, minor_max)| {
                        u64::from(minor_max - minor_min)
                            * u64::from(u32::max(major.abs_diff(major_min), major.abs_diff(major_max)))
                    })
                })
                .max()
        })
    }

    pub fn max_area(coords: Vec<Vec<i32>>) -> i64 {
        let mut buckets = HashMap::<u32, Vec<_>>::new();

        let coords = coords
            .into_iter()
            .map(|coord| {
                let [x, y] = coord.try_into().ok().unwrap();
                let x = x.cast_unsigned();
                let y = y.cast_unsigned();

                buckets
                    .entry(x)
                    .and_modify(|values| values.push(y))
                    .or_insert_with(|| vec![y]);

                (x, y)
            })
            .collect::<Vec<_>>();

        let candidate_1 = Self::check(&buckets);

        let mut recycled = buckets
            .drain()
            .map(|(_, mut values)| {
                values.clear();
                values
            })
            .collect::<Vec<_>>();

        for (x, y) in coords {
            buckets.entry(y).and_modify(|values| values.push(x)).or_insert_with(|| {
                recycled.pop().map_or_else(
                    || vec![x],
                    |mut buffer| {
                        buffer.push(x);

                        buffer
                    },
                )
            });
        }

        let candidate_2 = Self::check(&buckets);

        drop(buckets);

        candidate_1.max(candidate_2).unwrap_or(u64::MAX).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_area(coords: Vec<Vec<i32>>) -> i64 {
        Self::max_area(coords)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
