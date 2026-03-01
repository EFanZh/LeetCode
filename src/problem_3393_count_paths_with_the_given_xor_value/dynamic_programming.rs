pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::array;

impl Solution {
    pub fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const MODULUS: u32 = 1_000_000_007;

        let k = k.cast_unsigned() as usize;
        let columns = grid.first().map_or(0, Vec::len);
        let mut cache = vec![[0_u32; 16]; columns].into_boxed_slice();

        cache[0][0] = 1;

        for row in &grid {
            let mut left = &[0; 16];

            cache.iter_mut().zip(row).for_each(|(target, &value)| {
                *target = array::from_fn(|xor| {
                    let prev_xor = ((xor as i32 ^ value) & 15).cast_unsigned() as usize;
                    let count = left[prev_xor] + target[prev_xor];

                    if count >= MODULUS { count - MODULUS } else { count }
                });

                left = &*target;
            });
        }

        cache.last().unwrap()[k].cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::count_paths_with_xor_value(grid, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
