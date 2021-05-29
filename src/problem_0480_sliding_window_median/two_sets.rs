pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

#[derive(Default)]
struct MultiSet {
    inner: BTreeMap<i32, usize>,
    length: usize,
}

impl MultiSet {
    fn len(&self) -> usize {
        self.length
    }

    fn insert(&mut self, value: i32) {
        self.inner.entry(value).and_modify(|count| *count += 1).or_insert(1);

        self.length += 1;
    }

    fn remove(&mut self, value: i32) -> bool {
        match self.inner.entry(value) {
            Entry::Vacant(_) => false,
            Entry::Occupied(entry) => {
                if *entry.get() == 1 {
                    entry.remove();
                } else {
                    *entry.into_mut() -= 1;
                }

                self.length -= 1;

                true
            }
        }
    }

    fn first(&self) -> Option<i32> {
        self.inner.keys().next().copied()
    }

    fn last(&self) -> Option<i32> {
        self.inner.keys().next_back().copied()
    }
}

#[derive(Default)]
struct Window {
    left: MultiSet,
    right: MultiSet,
}

impl Window {
    fn insert(&mut self, value: i32) {
        let left = &mut self.left;
        let right = &mut self.right;

        if left.len() == right.len() {
            if let Some(x) = left.last().filter(|&x| x > value) {
                left.remove(x);
                left.insert(value);
                right.insert(x);
            } else {
                right.insert(value);
            }
        } else {
            let x = right.first().unwrap();

            if value > x {
                right.remove(x);
                right.insert(value);
                left.insert(x);
            } else {
                left.insert(value);
            }
        }
    }

    fn remove(&mut self, value: i32) {
        let left = &mut self.left;
        let right = &mut self.right;

        if left.len() == right.len() {
            if right.remove(value) {
                let x = left.last().unwrap();

                left.remove(x);
                right.insert(x);
            } else {
                left.remove(value);
            }
        } else if left.remove(value) {
            let x = right.first().unwrap();

            right.remove(x);
            left.insert(x);
        } else {
            right.remove(value);
        }
    }

    fn get_median(&self) -> f64 {
        if self.left.len() == self.right.len() {
            (f64::from(self.left.last().unwrap()) + f64::from(self.right.first().unwrap())) / 2.0
        } else {
            f64::from(self.right.first().unwrap())
        }
    }
}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as _;
        let mut result = Vec::with_capacity(nums.len() + 1 - k);
        let mut window = Window::default();
        let (head, body) = nums.split_at(k);

        for &num in head {
            window.insert(num);
        }

        result.push(window.get_median());

        for (&old, &new) in nums.iter().zip(body) {
            window.remove(old);
            window.insert(new);

            result.push(window.get_median());
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        Self::median_sliding_window(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
