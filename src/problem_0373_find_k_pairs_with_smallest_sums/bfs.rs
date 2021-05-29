pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn heap_push<T, K: Ord>(heap: &mut Vec<T>, value: T, mut get_key: impl FnMut(&T) -> K) {
        let mut i = heap.len();
        let key = get_key(&value);

        heap.push(value);

        loop {
            let parent = i.wrapping_sub(1) / 2;

            if let Some(parent_key) = heap.get(parent).map(&mut get_key) {
                if key < parent_key {
                    heap.swap(parent, i);
                    i = parent;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn heap_pop<T, K: Ord>(heap: &mut Vec<T>, mut get_key: impl FnMut(&T) -> K) -> Option<T> {
        heap.pop().map(|mut result| {
            if let Some(top) = heap.first_mut() {
                mem::swap(top, &mut result);

                let mut i = 0;
                let key = get_key(top);

                loop {
                    let left = i * 2 + 1;

                    if let Some(left_key) = heap.get(left).map(&mut get_key) {
                        let right = left + 1;

                        if let Some(right_key) = heap.get(right).map(&mut get_key) {
                            let (child, child_key) = if right_key < left_key {
                                (right, right_key)
                            } else {
                                (left, left_key)
                            };

                            if child_key < key {
                                heap.swap(i, child);
                                i = child;
                            } else {
                                break;
                            }
                        } else {
                            if left_key < key {
                                heap.swap(i, left);
                            }

                            break;
                        }
                    } else {
                        break;
                    }
                }
            }

            result
        })
    }

    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as _;
        let mut result = Vec::with_capacity(k);

        result.push(vec![nums1[0], nums2[0]]);

        let mut node = (0, 0);
        let mut queue = Vec::new();
        let get_key = |&(i, j): &(usize, usize)| nums1[i] + nums2[j];

        while result.len() != k {
            if node.1 != nums2.len() - 1 {
                Self::heap_push(&mut queue, (node.0, node.1 + 1), get_key);
            }

            if node.1 == 0 && node.0 != nums1.len() - 1 {
                Self::heap_push(&mut queue, (node.0 + 1, node.1), get_key);
            }

            if let Some((i, j)) = Self::heap_pop(&mut queue, get_key) {
                result.push(vec![nums1[i], nums2[j]]);
                node = (i, j);
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        Self::k_smallest_pairs(nums1, nums2, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
