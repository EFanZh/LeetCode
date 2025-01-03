pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::iter;

struct Item {
    end_index: Cell<usize>,
    sum: Cell<u64>,
}

impl Solution {
    fn filter(item: &Item) -> Option<(&Item, u64)> {
        let item_sum = item.sum.get();

        (item_sum != 0).then_some((item, item_sum))
    }

    pub fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
        let n = remove_queries.len();

        let items = iter::repeat_with(|| Item {
            sum: Cell::new(0),
            end_index: Cell::new(0),
        })
        .take(n)
        .collect::<Box<_>>();

        let mut result = vec![0; n];
        let mut max_sum = 0;

        result
            .iter_mut()
            .zip(&remove_queries)
            .rev()
            .for_each(|(target, &index)| {
                *target = max_sum as _;

                let mut index = index as u32 as usize;
                let mut end = &items[index];
                let mut sum = u64::from(nums[index] as u32);
                let other_end;

                'block: {
                    let (other, other_sum) = match (
                        items.get(index.wrapping_sub(1)).and_then(Self::filter),
                        items.get(index + 1).and_then(Self::filter),
                    ) {
                        (None, None) => {
                            other_end = end;

                            break 'block;
                        }
                        (None, Some((other, other_sum))) | (Some((other, other_sum)), None) => (other, other_sum),
                        (Some((left, left_sum)), Some((right, right_sum))) => {
                            index = left.end_index.get();
                            end = &items[index];
                            sum += left_sum;

                            (right, right_sum)
                        }
                    };

                    let other_end_index = other.end_index.get();

                    other_end = &items[other_end_index];

                    sum += other_sum;

                    end.end_index.set(other_end_index);
                    end.sum.set(sum);
                }

                other_end.end_index.set(index);
                other_end.sum.set(sum);

                max_sum = max_sum.max(sum);
            });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
        Self::maximum_segment_sum(nums, remove_queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
