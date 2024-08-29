pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroUsize;

impl Solution {
    pub fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
        let k = NonZeroUsize::new(k as u32 as _).unwrap();
        let mut lengths = Vec::new();
        let mut result = 0;

        for start in 0..k.get() {
            result += ((arr.len() - start - 1) / k + 1) as u32;

            let mut i = start;

            while let Some(&num) = arr.get(i) {
                i += k.get();

                let tail = lengths.partition_point(|&x| x <= num);

                if let Some(value) = lengths.get_mut(tail) {
                    *value = num;
                } else {
                    lengths.push(num);
                }
            }

            result -= lengths.len() as u32;
            lengths.clear();
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
