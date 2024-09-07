// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::btree_map::{Entry, OccupiedEntry};
use std::collections::{BTreeMap, VecDeque};
use std::mem;
use std::num::NonZeroU64;

#[derive(Default)]
struct MultiSet(BTreeMap<u32, u32>);

impl MultiSet {
    fn insert(&mut self, num: u32) {
        match self.0.entry(num) {
            Entry::Vacant(entry) => {
                entry.insert(0);
            }
            Entry::Occupied(entry) => *entry.into_mut() += 1,
        }
    }

    fn remove_entry(entry: OccupiedEntry<u32, u32>) {
        if *entry.get() == 0 {
            entry.remove();
        } else {
            *entry.into_mut() -= 1;
        }
    }

    fn remove(&mut self, num: u32) -> bool {
        match self.0.entry(num) {
            Entry::Vacant(_) => false,
            Entry::Occupied(entry) => {
                Self::remove_entry(entry);

                true
            }
        }
    }

    fn replace_first(&mut self, num: u32) -> Option<u32> {
        let entry = self.0.first_entry().unwrap();
        let key = *entry.key();

        #[expect(clippy::if_then_some_else_none, reason = "false positive")]
        if num > key {
            Self::remove_entry(entry);
            self.insert(num);

            Some(key)
        } else {
            None
        }
    }

    fn replace_last(&mut self, num: u32) -> Option<u32> {
        let entry = self.0.last_entry().unwrap();
        let key = *entry.key();

        #[expect(clippy::if_then_some_else_none, reason = "false positive")]
        if num < key {
            Self::remove_entry(entry);
            self.insert(num);

            Some(key)
        } else {
            None
        }
    }
}

enum Inner {
    Start {
        nums: Vec<u32>,
        required: u32,
        k: u32,
    },
    Running {
        queue: VecDeque<u32>,
        low: MultiSet,
        middle: MultiSet,
        high: MultiSet,
        middle_sum: u64,
        middle_length: NonZeroU64,
    },
}

pub struct MKAverage {
    inner: Inner,
}

impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        let m = m as u32;
        let k = k as u32;

        Self {
            inner: Inner::Start {
                nums: Vec::with_capacity(m as _),
                required: m,
                k,
            },
        }
    }

    fn count_nums(nums: &[u32]) -> MultiSet {
        let mut result = MultiSet::default();

        for &num in nums {
            result.insert(num);
        }

        result
    }

    fn add_element(&mut self, num: i32) {
        let num = num as u32;

        match &mut self.inner {
            Inner::Start { nums, required, k } => {
                nums.push(num);

                *required -= 1;

                if *required == 0 {
                    let m = nums.len();
                    let k = *k as usize;
                    let middle_length = m - k - k;
                    let mut buffer = nums.clone().into_boxed_slice();

                    buffer.select_nth_unstable(k - 1).2.select_nth_unstable(middle_length);

                    let (low, rest) = buffer.split_at(k);
                    let (middle, high) = rest.split_at(middle_length);

                    let middle_sum = middle.iter().fold(0, |sum, &value| sum + u64::from(value));
                    let queue = VecDeque::from(mem::take(nums));
                    let low = Self::count_nums(low);
                    let middle = Self::count_nums(middle);
                    let high = Self::count_nums(high);

                    drop(mem::replace(
                        &mut self.inner,
                        Inner::Running {
                            queue,
                            low,
                            middle,
                            high,
                            middle_sum,
                            middle_length: NonZeroU64::new(middle_length as _).unwrap(),
                        },
                    ));
                }
            }
            Inner::Running {
                queue,
                low,
                middle,
                high,
                middle_sum,
                ..
            } => {
                let old_num = queue.pop_front().unwrap();

                queue.push_back(num);

                if old_num != num {
                    if low.remove(old_num) {
                        let new_middle = high.replace_first(num).unwrap_or(num);

                        low.insert(middle.replace_first(new_middle).map_or(new_middle, |middle_min| {
                            *middle_sum -= u64::from(middle_min);
                            *middle_sum += u64::from(new_middle);

                            middle_min
                        }));
                    } else if high.remove(old_num) {
                        let new_middle = low.replace_last(num).unwrap_or(num);

                        high.insert(middle.replace_last(new_middle).map_or(new_middle, |middle_max| {
                            *middle_sum -= u64::from(middle_max);
                            *middle_sum += u64::from(new_middle);

                            middle_max
                        }));
                    } else {
                        middle.remove(old_num);
                        *middle_sum -= u64::from(old_num);

                        let new_middle = low
                            .replace_last(num)
                            .unwrap_or_else(|| high.replace_first(num).unwrap_or(num));

                        middle.insert(new_middle);
                        *middle_sum += u64::from(new_middle);
                    }
                }
            }
        }
    }

    fn calculate_mk_average(&self) -> i32 {
        match self.inner {
            Inner::Start { .. } => -1,
            Inner::Running {
                middle_sum,
                middle_length,
                ..
            } => (middle_sum / middle_length) as _,
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MKAverage for MKAverage {
    fn new(m: i32, k: i32) -> Self {
        Self::new(m, k)
    }

    fn add_element(&mut self, num: i32) {
        self.add_element(num);
    }

    fn calculate_mk_average(&self) -> i32 {
        self.calculate_mk_average()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MKAverage>();
    }
}
