pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::{NonZeroU16, NonZeroU32};

impl Solution {
    pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        let batch_size = NonZeroU16::new(batch_size as _).unwrap();

        assert!(batch_size.get() <= 9);

        let (greedy_result, group_states) = {
            let mut greedy_result = 0;
            let mut group_states = [0_u8; 8];

            for group in groups {
                let group = group as u32 % NonZeroU32::from(batch_size);

                if group == 0 {
                    greedy_result += 1;
                } else {
                    group_states[group as usize - 1] += 1;
                }
            }

            let mut iter = group_states[..usize::from(batch_size.get()) - 1].iter_mut();

            while let Some(left_count) = iter.next() {
                if let Some(right_count) = iter.next_back() {
                    let immediate_result = (*left_count).min(*right_count);

                    greedy_result += immediate_result;
                    *left_count -= immediate_result;
                    *right_count -= immediate_result;
                } else {
                    greedy_result += *left_count / 2;
                    *left_count %= 2;

                    break;
                }
            }

            (
                greedy_result,
                group_states.map(|count| NonZeroU16::new(u16::from(count) + 1).unwrap()),
            )
        };

        let (total_states, bases) = {
            let mut total_states = 1;

            let bases = group_states.map(|count| {
                let result = total_states;

                total_states *= count.get();

                NonZeroU16::new(result).unwrap()
            });

            (total_states, bases)
        };

        let mut cache = vec![0_u8; usize::from(total_states)].into_boxed_slice();

        for state in 1..total_states {
            let total_count = {
                let mut total_count = 0;
                let mut x = state;

                for (group_size, &count) in (1..).zip(&group_states[..usize::from(batch_size.get()) - 1]) {
                    total_count += group_size * (x % count);
                    x = x / count;
                }

                (total_count % batch_size) as u8
            };

            let max_count = {
                let mut max_count = 0;
                let mut state_iter = state;

                for (last_group, (&count, &base)) in (1..batch_size.get() as u8).zip(group_states.iter().zip(&bases)) {
                    if state_iter % count != 0 {
                        max_count =
                            max_count.max(cache[usize::from(state - base.get())] + u8::from(last_group == total_count));
                    }

                    state_iter = state_iter / count;
                }

                max_count
            };

            cache[usize::from(state)] = max_count;
        }

        u32::from(greedy_result + *cache.last().unwrap()) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        Self::max_happy_groups(batch_size, groups)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
