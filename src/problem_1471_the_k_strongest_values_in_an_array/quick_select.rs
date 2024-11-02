pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut arr = arr;
        let n = arr.len();
        let k = k as usize;

        if k < n {
            let median = *arr.select_nth_unstable((n - 1) / 2).1;

            arr.select_nth_unstable_by_key(k, |&value| Reverse(((value - median).abs(), value)));
            arr.truncate(k);
        }

        arr
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
        Self::get_strongest(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
