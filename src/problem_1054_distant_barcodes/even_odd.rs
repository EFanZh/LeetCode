pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn rearrange_barcodes(arr: Vec<i32>) -> Vec<i32> {
        let mut counts = HashMap::new();

        for &num in &arr {
            counts.entry(num).and_modify(|count| *count += 1).or_insert(1_u32);
        }

        let mut counts = counts.into_iter().collect::<Vec<_>>();

        let i = counts
            .iter()
            .enumerate()
            .max_by_key(|(_, (_, count))| *count)
            .unwrap()
            .0;

        counts.swap(0, i);

        let mut iter = counts
            .into_iter()
            .flat_map(|(value, count)| (0..count).map(move |_| value));

        let mut arr = arr;

        for target in arr.iter_mut().step_by(2) {
            *target = iter.next().unwrap();
        }

        for target in arr[1..].iter_mut().step_by(2) {
            *target = iter.next().unwrap();
        }

        arr
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rearrange_barcodes(arr: Vec<i32>) -> Vec<i32> {
        Self::rearrange_barcodes(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
