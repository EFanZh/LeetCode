pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Item {
    sum: u64,
    index: usize,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.sum.cmp(&self.sum)
    }
}

impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        let k = k as u32 as usize;
        let mut max_sum = 0;

        for num in &mut nums {
            if *num > 0 {
                max_sum += i64::from(*num as u32);
            } else {
                *num = -*num;
            }
        }

        if k < 2 {
            max_sum
        } else {
            let mut nums = nums.into_iter().map(|x| x as _).collect::<Vec<u32>>();

            let nums = if k < nums.len() {
                nums.select_nth_unstable(k - 1);

                &mut nums[..k]
            } else {
                nums.as_mut_slice()
            };

            nums.sort_unstable();

            let mut queue = BinaryHeap::new();

            let mut item = Item {
                sum: u64::from(nums[0]),
                index: 0,
            };

            for _ in 2..k {
                let next_index = item.index + 1;

                if let Some(&next) = nums.get(next_index) {
                    let next_sum = item.sum + u64::from(next);

                    queue.extend([
                        Item {
                            sum: next_sum - u64::from(nums[item.index]),
                            index: next_index,
                        },
                        Item {
                            sum: next_sum,
                            index: next_index,
                        },
                    ]);
                }

                item = queue.pop().unwrap();
            }

            max_sum - item.sum as i64
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        Self::k_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { sum: 2, index: 3 } == Item { sum: 2, index: 5 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
