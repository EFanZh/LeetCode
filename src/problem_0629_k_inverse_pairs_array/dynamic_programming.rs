pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const MODULUS: u32 = 1_000_000_007;
        let n = n as usize;
        let k = k as usize;
        let mut cache = vec![0_u32; k + 1];
        let mut temp = vec![0_u32; k + 1];

        cache[0] = 1;
        temp[0] = 1;

        for length in 2..=n {
            let max_inversions = length * (length - 1) / 2;
            let max_calculation_index = max_inversions / 2;
            let mut sum = 1;

            for (i, (target, num)) in temp
                .iter_mut()
                .zip(&cache)
                .enumerate()
                .take(max_calculation_index + 1)
                .skip(1)
            {
                sum += num + MODULUS - cache.get(i.wrapping_sub(length)).copied().unwrap_or(0);
                sum %= MODULUS;
                *target = sum;
            }

            if max_calculation_index < k {
                let (left, right) = temp.split_at_mut(max_calculation_index + 1);

                let left = if max_inversions % 2 == 0 {
                    &left[..left.len() - 1]
                } else {
                    left
                };

                for (target, source) in right.iter_mut().zip(left.iter().rev()) {
                    *target = *source;
                }
            }

            mem::swap(&mut cache, &mut temp);
        }

        cache[k] as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        Self::k_inverse_pairs(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
