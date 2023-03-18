pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Item {
    sum: i32,
    indices: [u8; 40],
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
        u32::cmp(&(other.sum as _), &(self.sum as _))
    }
}

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mat = mat.as_slice();

        assert!(mat.len() <= 40);

        let mut item = Item {
            sum: mat.iter().map(|row| row[0]).sum(),
            indices: [0; 40],
        };

        let mut queue = BinaryHeap::new();

        for _ in 1..k as u32 {
            for (i, row) in mat.iter().map(Vec::as_slice).enumerate() {
                let index = usize::from(item.indices[i]);

                if let Some(&new_value) = row.get(index + 1) {
                    item.indices[i] += 1;

                    queue.push(Item {
                        sum: item.sum - row[index] + new_value,
                        indices: item.indices,
                    });

                    item.indices[i] -= 1;
                }

                if index != 0 {
                    break;
                }
            }

            item = queue.pop().unwrap();
        }

        item.sum
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::kth_smallest(mat, k)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(
            Item {
                sum: 2,
                indices: [3; 40]
            } == Item {
                sum: 2,
                indices: [5; 40]
            },
        );

        assert!(
            Item {
                sum: 2,
                indices: [3; 40]
            } != Item {
                sum: 5,
                indices: [3; 40]
            },
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
