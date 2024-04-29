pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        const MODULUS: u64 = 1_000_000_007;

        let n = n as u32;
        let k = k as u32;
        let mut cache = vec![0_u64; k as _].into_boxed_slice();

        *cache.last_mut().unwrap() = 1;

        let (first, rest) = cache.split_first_mut().unwrap();

        for x in 1..u64::from(n) {
            let mut target = &mut *first;

            for next in &mut *rest {
                *target = (x * *target + *next) % MODULUS;
                target = next;
            }

            *target = (x * *target) % MODULUS;
        }

        cache[0] as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rearrange_sticks(n: i32, k: i32) -> i32 {
        Self::rearrange_sticks(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
