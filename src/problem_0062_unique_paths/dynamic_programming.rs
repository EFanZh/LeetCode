pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn unique_paths(mut m: i32, mut n: i32) -> i32 {
        if m > n {
            mem::swap(&mut m, &mut n);
        }

        let m = m as usize;

        // The grid has m rows and n columns.

        let mut cache = vec![1; m];

        for _ in 1..n {
            let mut prev = 0;

            for cache_item in &mut cache {
                *cache_item += prev;
                prev = *cache_item;
            }
        }

        cache.last().copied().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn unique_paths(m: i32, n: i32) -> i32 {
        Self::unique_paths(m, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
