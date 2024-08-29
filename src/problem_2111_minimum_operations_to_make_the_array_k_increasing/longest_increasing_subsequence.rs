pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroUsize;

impl Solution {
    pub fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
        let k = NonZeroUsize::new(k as u32 as _).unwrap();
        let mut lengths = Vec::new();
        let mut iter = arr.iter();
        let mut result = 0;

        for _ in 0..k.get() {
            let iter_2 = iter.clone().step_by(k.get());

            result += iter_2.len() as u32;

            for &num in iter_2 {
                let num = num as u32;
                let i = lengths.partition_point(|&x| x <= num);

                if let Some(value) = lengths.get_mut(i) {
                    *value = num;
                } else {
                    lengths.push(num);
                }
            }

            result -= lengths.len() as u32;
            lengths.clear();
            iter.next();
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
        Self::k_increasing(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
